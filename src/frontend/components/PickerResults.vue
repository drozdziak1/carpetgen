<template>
  <div id="picker-results">
    <h2>Results</h2>
    <div id="result-data">
      <p v-if="value === null">Select an area</p>
      <p v-else>
        North-west: {{ value.bounds.getNorthWest() }}<br />
        Zoom: {{ value.zoom }}<br />
      </p>
      <svg id="nw-tile"></svg>
    </div>
  </div>
</template>

<style lang="scss">
#picker-results {
  float: left;
  width: 50%;
}
</style>

<script>
import * as d3 from "d3";
import { geoPath, geoMercator } from "d3-geo";
import { tile as d3Tile } from "d3-tile";
import { buffer as d3Buffer } from "d3-fetch";

import * as Promise from "bluebird";
import Protobuf from "pbf";
import { VectorTile } from "@mapbox/vector-tile";

export default {
  data: function() {
    return {};
  },
  props: {
    value: Object
  },
  computed: {
    nwTileCoords: function() {
      var { bounds, zoom } = this.value;
      var nw = bounds.getNorthWest();
      return { x: long2tile(nw.lng, zoom), y: lat2tile(nw.lat, zoom), z: zoom };
    }
  },
  methods: {
    long2tile: function(lon, zoom) {
      return Math.floor(((lon + 180) / 360) * Math.pow(2, zoom));
    },
    lat2tile: function(lat, zoom) {
      return Math.floor(
        ((1 -
          Math.log(
            Math.tan((lat * Math.PI) / 180) +
              1 / Math.cos((lat * Math.PI) / 180)
          ) /
            Math.PI) /
          2) *
          Math.pow(2, zoom)
      );
    }
  },
  watch: {
    value: function(newVal, oldVal) {
      var width = 300;
      var height = 300;
      var svg = d3.select("#nw-tile").attr("viewBox", [0, 0, width, height]);

      var nw = newVal.bounds.getNorthWest();
      var zoom = newVal.zoom;

      var projection = geoMercator()
        .center([nw.lng, nw.lat])
        .scale(Math.pow(2, zoom + 9) / (2 * Math.PI))
        .translate([width / 2, height / 2])
        .precision(0);

      svg
        .append("rect")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("fill", "#f6f4e7");

      var tile = d3Tile()
        .size([width, height])
        .scale(projection.scale() * 2 * Math.PI)
        .translate(projection([0, 0]))
        .tileSize(512);

      var tiles = Promise.all(
        tile().map(async d => {
console.log("tile zoom", d[2])
          const tileUrl = `http://localhost:8080/data/v3/${d[2]}/${d[0]}/${
            d[1]
          }.pbf`;
          d.layers = new VectorTile(
            new Protobuf(await d3Buffer(tileUrl))
          ).layers;
          d.url = tileUrl;
          return d;
        })
      );

      tiles.map(d => {
        drawRoads(svg, d);
      });

      function drawRoads(svg, d) {
        const g = svg.append("g");

        g.attr("fill", "none")
          .attr("stroke-linecap", "round")
          .attr("stroke-linejoin", "round");

        const features = filter(geojson(d, d.layers.transportation), p => {
          return (
            !p.properties.bridge &&
            !p.properties.tunnel
            /* &&
             *(p.properties.tracktype === 1 ||
             *  p.properties.class.match(
             *    /^(motorway|motorway_link|trunk|trunk_link|primary|primary_link|secondary|secondary_link|tertiary|tertiary_link|road|unclassified|residential|pedestrian|living_street|service)$/
             *  ))
             */
          );
        });

        g.append("path")
          .datum(features)
          .attr("d", geoPath().projection(projection))
          .attr("stroke", "#777")
          .attr("stroke-width", "6");

        g.append("path")
          .datum(features)
          .attr("d", d3.geoPath().projection(projection))
          .attr("stroke", "#fff")
          .attr("stroke-width", "4");
      }

      function geojson([x, y, z], layer, filter = () => true) {
        if (!layer) return;
        const features = [];
        for (let i = 0; i < layer.length; ++i) {
          const f = layer.feature(i).toGeoJSON(x, y, z);
          if (filter.call(null, f, i, features)) features.push(f);
        }
        var ret = { type: "FeatureCollection", features };
        console.log(ret);
        return ret;
      }

      function filter({ features }, test) {
        return { type: "FeatureCollection", features: features.filter(test) };
      }
    }
  }
};
</script>

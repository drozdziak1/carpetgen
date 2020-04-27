<template>
  <div id="picker-results">
    <h2>Results</h2>
    <div id="result-data">
      <p v-if="value === null">Select an area</p>
      <div v-else>
        <p>
          Center: {{ value.bounds.getCenter() }}<br />
          Zoom: {{ value.zoom }} + {{ zoomDelta }}<br />
        </p>
        <button v-on:click="incZoom">Detail +</button>
        <button v-on:click="decZoom">Detail -</button>
        <svg id="center-tile"></svg>
      </div>
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

import aspectToWH from "../util.js";

export default {
  data: function() {
    return {
      zoomDelta: 0
    };
  },
  props: {
    value: Object,
    unit: Number
  },
  methods: {
    incZoom: function() {
      if (this.value.zoom + this.zoomDelta < 14) {
        this.zoomDelta += 1;
      }
    },
    decZoom: function() {
      if (this.value.zoom + this.zoomDelta > 7) {
        this.zoomDelta -= 1;
      }
    },
    svgRenderTiles: function(bounds, zoom, zoomDelta, width, height) {
      console.log(`width: ${width}, height: ${height}`);
      var svg = d3
        .select("#center-tile")
        .attr("viewBox", [0, 0, width, height])
        .attr("width", width)
        .attr("height", height);

      var center = bounds.getCenter();

      var projection = geoMercator()
        .center([center.lng, center.lat])
        .scale(Math.pow(2, zoom + 7) / (2 * Math.PI))
        .translate([width / 2, height / 2])
        .precision(0);

      svg
        .append("rect")
        .attr("width", width)
        .attr("height", height)
        .attr("fill", "#f6f4e7");

      var tile = d3Tile()
        .size([width, height])
        .scale(projection.scale() * Math.PI * 2)
        .translate(projection([0, 0]))
        .tileSize(256)
        .zoomDelta(zoomDelta);

      var tiles = Promise.all(
        tile().map(async d => {
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
        drawLandUse(svg, d);
        drawRoads(svg, d);
        drawBuildings(svg, d);
      });

      function drawRoads(svg, d) {
        const g = svg.append("g");

        g.attr("fill", "none")
          .attr("stroke-linecap", "round")
          .attr("stroke-linejoin", "round");

        const features = filter(geojson(d, d.layers.transportation), p => {
          return (
            !p.properties.bridge && !p.properties.tunnel
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
          .attr("stroke", "#777");
        /*.attr("stroke-width", "1");*/

        g.append("path")
          .datum(features)
          .attr("d", d3.geoPath().projection(projection))
          .attr("stroke", "#fff");
        /*.attr("stroke-width", "3");*/
      }

      function drawBuildings(svg, d) {
        const g = svg.append("g");
        g.append("path")
          .datum(geojson(d, d.layers.building))
          .attr("d", d3.geoPath().projection(projection))
          .attr("fill", "#888");
      }

      function drawLandUse(svg, d) {
        const g = svg.append("g");
        g.append("path")
          .datum(geojson(d, d.layers.landuse))
          .attr("d", d3.geoPath().projection(projection))
          .attr("fill", "#ddd");
      }

      function geojson([x, y, z], layer, filter = () => true) {
        if (!layer) return;
        const features = [];
        for (let i = 0; i < layer.length; ++i) {
          const f = layer.feature(i).toGeoJSON(x, y, z);
          if (filter.call(null, f, i, features)) features.push(f);
        }
        var ret = { type: "FeatureCollection", features };
        return ret;
      }

      function filter({ features }, test) {
        return { type: "FeatureCollection", features: features.filter(test) };
      }
    }
  },
  watch: {
    "value.bounds": function(newVal, oldVal) {
      var { width, height } = aspectToWH(
        this.value.aspectRatio,
        this.value.isVertical,
        this.unit
      );
      this.svgRenderTiles(
        newVal,
        this.value.zoom,
        this.zoomDelta,
        width,
        height
      );
    },
    "value.zoom": function(newVal, oldVal) {
      var { width, height } = aspectToWH(
        this.value.aspectRatio,
        this.value.isVertical,
        this.unit
      );
      this.svgRenderTiles(
        this.value.bounds,
        newVal,
        this.zoomDelta,
        width,
        height
      );
    },
    "value.aspectRatio": function(newVal, oldVal) {
      var { width, height } = aspectToWH(
        newVal,
        this.value.isVertical,
        this.unit
      );
      this.svgRenderTiles(
        this.value.bounds,
        this.value.zoom,
        this.zoomDelta,
        width,
        height
      );
    },
    "value.isVErtical": function(newVal, oldVal) {
      var { width, height } = aspectToWH(newVal, newVal, this.unit);
      this.svgRenderTiles(
        this.value.bounds,
        this.value.zoom,
        this.zoomDelta,
        width,
        height
      );
    },
    zoomDelta: function(newVal, oldVal) {
      var { width, height } = aspectToWH(
        this.value.aspectRatio,
        this.value.isVertical,
        this.unit
      );
      this.svgRenderTiles(
        this.value.bounds,
        this.value.zoom,
        newVal,
        width,
        height
      );
    }
  }
};
</script>

<template>
  <div id="map" ref="mapEl"></div>
</template>

<script>
import L from "leaflet";
import "leaflet-areaselect/src/leaflet-areaselect.js";

export default {
  data() {
    return {
      areaSelect: null,
      map: null,
      bounds: null
    };
  },
  computed: {
    dimensions: function() {
      var splitRatio = this.aspectRatio.split(":");
      var n = splitRatio[0];
      var d = splitRatio[1];

      return {
        width: this.isVertical ? this.unit : (this.unit * n) / d,
        height: this.isVertical ? (this.unit * n) / d : this.unit
      };
    }
  },
  props: {
    aspectRatio: String,
    isVertical: Boolean,
    unit: Number
  },
  watch: {
    aspectRatio: function() {
      this.areaSelect.setDimensions(this.dimensions);
    },
    isVertical: function() {
      // Not using this.dimensions because it messes up the scaling
      this.areaSelect.setDimensions({
        width: this.areaSelect._height,
        height: this.areaSelect._width
      });
    }
  },
  mounted() {
    // Prepare a map with areaSelect
    var map = L.map(this.$refs["mapEl"]).setView([52.237049, 21.017532], 11);

    L.tileLayer("http://localhost:8080/styles/basic-preview/{z}/{x}/{y}.png", {
      attribution:
        'Tiles generated with <a href="https://openmaptiles.org/">OpenMapTiles</a> | Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>'
    }).addTo(map);

    var splitRatio = this.aspectRatio.split(":");
    var n = splitRatio[0];
    var d = splitRatio[1];

    var w = this.isVertical ? this.unit : (this.unit * n) / d;
    var h = this.isVertical ? (this.unit * n) / d : this.unit;
    var areaSelect = L.areaSelect({
      width: w,
      height: h,
      keepAspectRatio: true
    });
    areaSelect.addTo(map);

    var bounds = areaSelect.getBounds();

    console.log("Bounds: ", bounds);

    var comp = this;
    areaSelect.on("change", function() {
      var bounds = this.getBounds();
      console.log("Bounds:", bounds);
      console.log("Zoom: ", map.getZoom());
      comp.$emit("area-change", { bounds: bounds, zoom: map.getZoom() });
    });

    this.areaSelect = areaSelect;
    this.map = map;
  }
};
</script>

<template>
<div id="map" ref="mapEl"></div>
</template>

<script>
import L from 'leaflet';
import 'leaflet-areaselect/src/leaflet-areaselect.js';


export default {
  data() {
    return {
areaSelect: null,
	      map: null
    }
  },
props: {
aspectRatio: String,
	     isVertical: Boolean,
	     unit: Number,
       },
watch: {
aspectRatio: function(newRatio, oldRatio) {
	       var splitRatio = newRatio.split(":");
	       var n = splitRatio[0]; // numerator
	       var d = splitRatio[1]; // denominator

	       var w = this.isVertical ? this.unit : this.unit * n / d;
	       var h = this.isVertical ? this.unit * n / d : this.unit;
	       this.areaSelect.setDimensions({width: w, height: h });
	     },
isVertical: function(newValue, oldValue) {
	      // Flip areaSelect's dimensions when isVertical is toggled
	      this.areaSelect.setDimensions({
width: this.areaSelect._height,
height: this.areaSelect._width
});
}
},
  mounted() {

    var map = L.map(this.$refs['mapEl']).setView([52.237049, 21.017532], 11);

    L.tileLayer('http://localhost:8080/styles/basic-preview/{z}/{x}/{y}.png',
	{
attribution: 'Tiles generated with <a href="https://openmaptiles.org/">OpenMapTiles</a> | Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>'
}).addTo(map);

var splitRatio = this.aspectRatio.split(":");
var n = splitRatio[0];
var d = splitRatio[1];

var w = this.isVertical ? this.unit : this.unit * n / d;
var h = this.isVertical ? this.unit * n / d : this.unit;
var areaSelect = L.areaSelect({width: w, height: h, keepAspectRatio: true});
areaSelect.addTo(map);

var bounds = areaSelect.getBounds();

console.log("Bounds: ", bounds);

// Get a callback when the bounds change
areaSelect.on("change", function() {
    console.log("Bounds:", this.getBounds());
    });
this.areaSelect = areaSelect;
this.map = map;
},
  }
</script>
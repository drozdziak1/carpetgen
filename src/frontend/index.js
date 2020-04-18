import L from 'leaflet';
import 'leaflet-areaselect/src/leaflet-areaselect.js';
import './styles/main.scss';

var map = L.map('map').setView([52.237049, 21.017532], 11);

L.tileLayer('http://localhost:8080/styles/basic-preview/{z}/{x}/{y}.png',
  {
    attribution: 'Tiles generated with <a href="https://openmaptiles.org/">OpenMapTiles</a> | Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>'
  }).addTo(map);


var areaSelect = L.areaSelect({width: 400, height: 400, keepAspectRatio: true});
areaSelect.addTo(map);


var bounds = areaSelect.getBounds();

console.log("Bounds: ", bounds);

// Get a callback when the bounds change
areaSelect.on("change", function() {
    console.log("Bounds:", this.getBounds());
});

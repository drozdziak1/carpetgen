<template>
  <div id="area-picker" ref="areaPickerEl">
    <h2>Carpet shape and orientation</h2>
    <select id="carpet-shape-dropdown" v-model="aspectRatio">
      <option value="1:1">1:1</option>
      <option value="2:1">2:1</option>
      <option value="4:3">4:3</option>
    </select>
    <label><input type="checkbox" v-model="isVertical" /> Vertical</label>
    <Map
      :aspect-ratio="aspectRatio"
      :is-vertical="isVertical"
      :unit="200"
      @area-change="updateAreaData"
    />
  </div>
</template>

<style lang="scss">
#area-picker {
  float: left;
  width: 50%;
}
</style>

<script>
import Map from "./Map.vue";
export default {
  data() {
    return {
      aspectRatio: "4:3",
      isVertical: true,
    };
  },
  components: {
    Map
  },
  methods: {
    updateAreaData: function(areaData) {
      this.$emit("input", areaData);
    }
  }
};
</script>

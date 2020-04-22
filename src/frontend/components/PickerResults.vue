<template>
  <div id="picker-results">
    <h2>Results</h2>
    <div id="result-data">
      <p v-if="value === null">Select an area</p>
      <p v-else>
        North-west: {{ value.bounds.getNorthWest() }}<br />
        Zoom: {{ value.zoom }}
      </p>
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
export default {
  data: function() {
    return {};
  },
  props: {
    value: Object
  }
};
</script>

<template>
  <div id="app">
    <h1>CarpetGen</h1>
    <AreaPicker v-model="areaData" :min-zoom="minZoom" :max-zoom="maxZoom" />
    <PickerResults
      v-model="areaData"
      :min-zoom="minZoom"
      :max-zoom="maxZoom"
      :unit="200"
    />
  </div>
</template>

<script>
import AreaPicker from "./AreaPicker.vue";
import PickerResults from "./PickerResults.vue";

export default {
  components: {
    AreaPicker,
    PickerResults
  },
  data: function() {
    return {
      areaData: null,
      minZoom: 7,
      maxZoom: 14,
    };
  }
};
</script>

<template>
  <div id="app">
    <h1>CarpetGen</h1>
    <AreaPicker v-model="areaData" />
    <PickerResults v-model="areaData" />
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
    };
  }
};
</script>

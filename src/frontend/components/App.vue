<template>
  <div id="app">
    <h1>CarpetGen</h1>
    <AreaPicker />
    <PickerResults />
  </div>
</template>

<script>
import AreaPicker from "./AreaPicker.vue";
import PickerResults from "./PickerResults.vue";

export default {
  components: {
    AreaPicker,
    PickerResults
  }
};
</script>

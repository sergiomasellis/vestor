<template>
  <span :class="[isPossitiveOrNegative]">
    <!-- NUMBER + or - plus $ -->

    <!-- PERCENTAGES UP or DOWN  -->
    <!-- {{$filters.toCurrency(textValue, true)}} -->
    <template v-if="type === 'percentage'">{{toPercentageConverter(value)}}</template>
    <template v-else>{{toCurrencyConverter(value, true)}}</template>
  </span>
</template>

<script lang="ts">
import { defineComponent, computed } from "vue";
import toCurrency from "../utils/toCurrency";
import toPercentage from "../utils/toPercentage";

export default defineComponent({
  props: {
    value: Number,
    type: String, // percentage or default which is currency/number
  },
  setup(props) {

    // used to add green or red color
    const isPossitiveOrNegative = computed(() => {
      let positiveCash = props.value > 0;
      return {
        isPositive: positiveCash,
        isNegative: !positiveCash,
      };
    });

    // only needed for number types
    const toCurrencyConverter = (value, optionalPrepend = false) => {
      return toCurrency(value, optionalPrepend);
    };

    // only needed for percentage types
    const toPercentageConverter = (value) => {
        return toPercentage(value);
    }

    return {
      isPossitiveOrNegative,
      toCurrencyConverter,
      toPercentageConverter
    };
  },
});
</script>

<style lang="scss" scoped>
</style>
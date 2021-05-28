<template>
  <div class="account-balance">
    <sd-button theme="default" flat size="sm" iconEnd="info">
      Current Value
      <sd-tooltip>Changes in account value/return calculations may occur throughout the day due to several factors including market movements, your deposits and withdrawals, and the availability of third party data and are not necessarily a reflection of actual account value. vestor prepared this dashboard for informational purposes and your convenience, but your vestor account statement is the only official record of your actual account value and holdings.</sd-tooltip>
    </sd-button>
    <h2 class="account-balance__title">{{toCurrency(accountBalance)}}</h2>
    <sd-row class="account-balance__content">
      <sd-col>Starting value</sd-col>
      <sd-col>{{dateOfAccountCreation}}</sd-col>
    </sd-row>
    <sd-row class="account-balance__content">
      <sd-col class="account-balance__muted-text">Net cash flow</sd-col>
      <sd-col><net-indicator-text :value="netCashFlow" /> </sd-col>
    </sd-row>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from "vue";
import toCurrency from '../utils/toCurrency';
import NetIndicatorText from "./NetIndicatorText.vue";

export default defineComponent({
  components: { NetIndicatorText },
  props: {
    accountBalance: Number,
    dateOfAccountCreation: String,
    netCashFlow: Number,
  },
  setup(props) {

    return {
      toCurrency
    };
  },
});
</script>

<style lang="scss" scoped>
.account-balance {
  &__title {
    font-size: 2.5rem;
  }
  &__content {
    color: var(--text-accent);
    font-size: 0.8rem;
  }
}
</style>
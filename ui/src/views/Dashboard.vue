<template>
  <section class="dashboard">
    <navigation />

    <!-- HERO DASH -->
    <section class="hero">
      <sd-container>
        <sd-row>
          <sd-col :xs="12" :md="7">
            <h1>Dashboard</h1>
            <!-- Account Balance component -->
            <account-balance
              :accountBalance="accountBalance"
              :dateOfAccountCreation="dateOfAccountCreation"
              :netCashFlow="netCashFlow"
            />
          </sd-col>
          <sd-col>
            <sd-card>
              <sd-card-header />
              <sd-card-body>
                <div class="addCash">
                  <sd-row>
                    <sd-col :sm="6">
                      <h5>Cash Balance</h5>
                      <h3>
                        <strong>{{toCurrency(cashBalance)}}</strong>
                      </h3>
                    </sd-col>
                    <sd-col>
                      <sd-button block theme="secondary" size="lg">Add C4$H</sd-button>
                    </sd-col>
                  </sd-row>
                  <hr class="sd--divider" />
                  <sd-row>
                    <sd-col>
                      <sd-button theme="default" flat size="sm" iconEnd="info">
                        Auto-Invest
                        <sd-tooltip>Auto Investing is the shit</sd-tooltip>
                      </sd-button>
                    </sd-col>
                    <sd-col>
                      <sd-switch />
                    </sd-col>
                  </sd-row>
                  <sd-row>
                    <sd-col>
                      <sd-field icon="money" required placeholder label="Minimum cash balance" />
                    </sd-col>
                  </sd-row>
                </div>
              </sd-card-body>
            </sd-card>
          </sd-col>
        </sd-row>
      </sd-container>
    </section>

    <section class="dashboard__container">
      <sd-container>
        <sd-row>
          <sd-col class="dashboard__datefilter">
            <sd-button
              v-for="(filter, index) in accountHistoryFilter"
              :key="index"
              size="xs"
              :theme="(filter.state) ? 'primary': 'dark'"
              @click="setFilterBy(index)"
              :active="(filter.state) ? true : false"
            >{{filter.label}}</sd-button>
          </sd-col>
        </sd-row>
        <sd-row>
          <!-- PIE WIDGET -->
          <sd-col :sm="4">
            <sd-card>
              <sd-card-body>
                <sd-row class="dailyPieChart">
                  <sd-col>
                    <div class="gains">
                      <h5>
                        <strong>{{toCurrency(dailyAccountBalance)}}</strong>
                      </h5>
                      <small>
                        <net-indicator-text :value="dailyCashFlow" />&nbsp;
                        <net-indicator-text :value="dailyNetPercentage" type="percentage" />
                      </small>
                    </div>
                    <vue3-chart-js
                      :id="doughnutChart.id"
                      :type="doughnutChart.type"
                      :data="doughnutChart.data"
                      :options="doughnutChart.options"
                    ></vue3-chart-js>
                  </sd-col>
                </sd-row>
                <sd-row>
                  <sd-col>
                    <sd-button>Buy/Sell</sd-button>
                    <sd-button outline>Rebalance</sd-button>
                    <sd-button outline>Edit</sd-button>
                  </sd-col>
                </sd-row>
              </sd-card-body>
            </sd-card>
          </sd-col>
          <sd-col :sm="8">
            <sd-card>
              <sd-card-body>
                <vue3-chart-js ref="accountHistoryChartRef" v-bind="{...accountHistoryChart}"></vue3-chart-js>
              </sd-card-body>
            </sd-card>
          </sd-col>
        </sd-row>
      </sd-container>
    </section>
  </section>
</template>

<script lang="ts">
import {
  ref,
  reactive,
  toRefs,
  computed,
  watchEffect,
  defineComponent,
} from "vue";
import Navigation from "../components/Navigation.vue";
import AccountBalance from "../components/AccountBalance.vue";
import Vue3ChartJs from "@j-t-mcc/vue3-chartjs";
import toCurrency from "../utils/toCurrency";
import NetIndicatorText from "../components/NetIndicatorText.vue";

export default defineComponent({
  components: { Navigation, Vue3ChartJs, AccountBalance, NetIndicatorText },
  setup() {
    const state = reactive({
      accountBalance: 100000.95,
      cashBalance: 1000.25,
      netCashFlow: 20000.45,
      dailyAccountBalance: 100000.95,
      dailyCashFlow: 15433.23,
      dailyNetPercentage: 0.0578,
      dateOfAccountCreation: "Jun, 15, 2018",
      account_history: [10000, 15000, 25000, 23500, 90000, 100000],
      autoInvest: false,
      accountHistoryFilter: [
        { label: "1D", state: true },
        { label: "1W", state: false },
        { label: "1M", state: false },
        { label: "1Q", state: false },
        { label: "1Y", state: false },
      ],
      accountHistoryFilterActive: 0,
    });

    const setFilterBy = (index) => {
      // reset current 
      state.accountHistoryFilter[state.accountHistoryFilterActive].state = false;
      state.accountHistoryFilter[index].state = !state.accountHistoryFilter[index].state;
      state.accountHistoryFilterActive = index;
    };


    const oneDay = () => {};

    const accountHistoryChartRef = ref(null);

    const accountHistoryChart = {
      id: "line",
      type: "line",
      data: {
        labels: Array(100).fill(" "),
        datasets: [
          {
            label: "Value overtime",
            data: [10000, 15000, 25000, 23500, 90000, 100000],
            backgroundColor: ["#83cf2a"],
            borderColor: ["#83cf2a"],
            borderWidth: 1,
            fill: false,
          },
        ],
      },
      options: {
        responsive: true,
        legend: { display: false },
        elements: {
          line: {
            tension: 0.3,
          },
        },
        scales: {
          xAxes: [
            {
              gridLines: {
                show: false,
              },
            },
          ],
          yAxes: [
            {
              ticks: {
                callback: function (value, index, values) {
                  return value.toLocaleString("en-US", {
                    style: "currency",
                    currency: "USD",
                  });
                },
              },
              gridLines: {
                show: false,
              },
            },
          ],
        },
      },
    };

    const doughnutChart = {
      id: "doughnut",
      type: "doughnut",
      data: {
        labels: ["VOO", "VEA", "SPY", "NIO"],
        datasets: [
          {
            backgroundColor: ["#ab40fa", "#f8c200", "#d0691e", "#c25361"],
            data: [40, 20, 80, 10],
            borderWidth: 0,
            borderColor: "#636065",
            hoverOffset: -10,
            borderRadius: 5,
          },
        ],
      },
      options: {
        cutout: "80%",
        clip: { left: 16, top: 16, right: 16, bottom: 16 },
        plugins: {
          legend: {
            display: false,
            labels: {
              color: "rgb(255, 99, 132)",
            },
          },
        },
      },
    };


    return {
      ...toRefs(state),
      accountHistoryChartRef,
      doughnutChart,
      accountHistoryChart,
      oneDay,
      toCurrency,
      setFilterBy
    };
  },
});
</script>

<style lang="scss" scoped>
.dashboard {
  &__container {
    margin-top: -120px;
  }

  &__datefilter {
    justify-content: flex-end;
    display: flex;
  }

  .hero {
    padding-top: 16px;
    background-color: #161616;
    background-image: url("dashboard-bg.svg");
    padding-bottom: 140px;
  }

  h1 {
    font-size: 3rem;
    font-weight: 400;
    padding-top: 36px;
  }

  #doughnut {
    padding: 16px;
  }

  .gains {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
  }
}
</style>
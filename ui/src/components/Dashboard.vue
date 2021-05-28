<template>
  <section class="dashboard">
    <navigation />

    <!-- HERO DASH -->
    <section class="hero">
      <sd-container>
        <sd-row>
          <sd-col :xs="12" :md="7">
            <h1>Dashboard</h1>
            <div class="balance">
              <sd-button theme="default" flat size="sm" iconEnd="info">
                Current Value
                <sd-tooltip>Changes in account value/return calculations may occur throughout the day due to several factors including market movements, your deposits and withdrawals, and the availability of third party data and are not necessarily a reflection of actual account value. vestor prepared this dashboard for informational purposes and your convenience, but your vestor account statement is the only official record of your actual account value and holdings.</sd-tooltip>
              </sd-button>
              <h2 class="balance__title">{{$filters.toCurrency(accountBalance)}}</h2>
              <sd-row class="balance__content">
                <sd-col>Starting value</sd-col>
                <sd-col>{{dateOfAccountCreation}}</sd-col>
              </sd-row>
              <sd-row class="balance__content">
                <sd-col class="balance_muted-text">Net cash flow</sd-col>
                <sd-col :class="[computedCashFlow]">{{$filters.toCurrency(netCashFlow, true)}}</sd-col>
              </sd-row>
            </div>
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
                        <strong>{{$filters.toCurrency(cashBalance)}}</strong>
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
            <sd-button theme="primary" size="xs">1D</sd-button>
            <sd-button theme="dark" size="xs" @click="oneWeek">1W</sd-button>
            <sd-button theme="dark" size="xs" @click="oneMonth">1M</sd-button>
            <sd-button theme="dark" size="xs">1Q</sd-button>
            <sd-button theme="dark" size="xs">1Y</sd-button>
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
                        <strong>{{$filters.toCurrency(dailyAccountBalance)}}</strong>
                      </h5>
                      <small
                        :class="[computedCashFlow]"
                      >{{$filters.toCurrency(dailyCashFlow, true)}} ▲▼ 50.95%</small>
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
          <sd-col>
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
import Navigation from "./Navigation.vue";
import Vue3ChartJs from "@j-t-mcc/vue3-chartjs";

export default defineComponent({
  components: { Navigation, Vue3ChartJs },
  setup() {
    const state = reactive({
      accountBalance: 100000.95,
      cashBalance: 1000.25,
      netCashFlow: 20000.45,
      dailyAccountBalance: 100000.95,
      dailyCashFlow: 15433.23,
      dateOfAccountCreation: "Jun, 15, 2018",
      account_history: [10000, 15000, 25000, 23500, 90000, 100000],
      autoInvest: false,
    });

    const computedCashFlow = computed(() => {
      let positiveCash = state.netCashFlow > 0;
      return {
        isPositive: positiveCash,
        isNegative: !positiveCash,
      };
    });

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

    const oneWeek = () => {
      console.log(accountHistoryChart.data.datasets.data);
      accountHistoryChart.data.datasets[0].data = [
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
      ];
      accountHistoryChartRef.value.update(250);
      console.log(accountHistoryChart.data.datasets.data);
    };

    const oneMonth = () => {
      const chartDataSet = accountHistoryChart.data.datasets[0];
      accountHistoryChart.data.labels = [
        "ine",
        "ine",
        "ine",
        "ine",
        "ine",
        "ine",
        "ine",
      ];

      chartDataSet.data = [
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
        Math.floor(Math.random() * 500),
      ];
      accountHistoryChartRef.value.update(600);
      console.log(accountHistoryChart.data.datasets.data);
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
      computedCashFlow,
      doughnutChart,
      accountHistoryChart,
      oneWeek,
      oneMonth,
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

  .isPositive {
    color: var(--success-highlight);
  }
  .isNegative {
    color: var(--danger);
  }

  h1 {
    font-size: 3rem;
    font-weight: 400;
    padding-top: 36px;
  }

  .balance {
    &__title {
      font-size: 2.5rem;
    }
    &__content {
      color: var(--text-accent);
      font-size: 0.8rem;
    }
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
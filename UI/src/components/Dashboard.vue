<template>
  <div class="dashboard">
    <h1>DASHBOARD</h1>
    <div id="dashchart" class="dashboard__chart"></div>
    {{data?.value}}
  </div>
</template>

<script>
// import * as d3 from "d3";
import { onMounted, computed, ref } from "vue";
export default {
  setup() {
    const data = ref(null);
    const loading = ref(true);
    const error = ref(null);

    function fetchData() {
      loading.value = true;

      return fetch("http://localhost:8081/articles", {
        method: 'get',
        headers: {
          'Content-Type': 'application/json'
        }
      }).then((res) => {
        if(!res.ok) {
          const error = new Error(res.statusText);
          error.josn = res.json();
          throw error;
        }
        return res.json();
      }).then(json => {
        data.value = json.data;
      })
      
    }

    onMounted(() => {
      fetchData();
    });

    return {
      data,
      loading,
      error
    };
  },
};
</script>

<style>
</style>
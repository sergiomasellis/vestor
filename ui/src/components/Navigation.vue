<template>
  <nav class="navigation">
    <sd-container class="navigation__container">
      <ul>
        <li>
          <sd-icon color="var(--success)" name="accessibility_new" size="xl" />
        </li>
        <li>
          <sd-button flat pill>INVEST</sd-button>
        </li>
        <li>
          <sd-button flat pill>RESEARCH</sd-button>
        </li>
        <li>
          <sd-button flat pill>TRANSFER</sd-button>
        </li>
      </ul>
      <ul>
        <li>
          <sd-button outline rounded icon="account_circle">Sergio</sd-button>
        </li>
      </ul>
    </sd-container>
  </nav>
</template>

<script>
import { ref } from "vue";
export default {
  setup() {
    const data = ref(null);
    const loading = ref(true);
    const error = ref(null);

    return {
      data,
      loading,
      error,
    };
  },
};
</script>

<style  lang="scss" scoped>
.navigation {
  background: var(--background-opaque);

  &__container {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  ul {
    display: flex;
    text-decoration: none;
    list-style: none;
    margin: 0;
    padding: 0;
    li a {
      margin: 16px 8px;
      color: red;
    }
  }
}
</style>
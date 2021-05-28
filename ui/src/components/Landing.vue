<template>
  <sd-container class="landing">
    <sd-row>
      <sd-col :lg="4">
        <img :src="`../test2.jpg`" alt />
        <h1>Welcome to Vestor</h1>
        <h2>Start investing commsion-free</h2>
        <small>
          Other fees may apply. view our
          <a>fee schedule</a> to learn more.
          <br />All investments have risks.
        </small>
      </sd-col>
    </sd-row>
    <sd-row>
      <sd-col :sm="12" :md="12" :lg="4">
        <sd-button block pill size="sm" class="landing__signup">Sign up</sd-button>
        <sd-button block pill flat size="sm" @click="handleDialogOpen()">Log in</sd-button>
      </sd-col>
    </sd-row>
    <sd-dialog aside v-model:active="modal">
      <sd-dialog-header>
        <sd-button flat @click="handleDialogClose()">
          <sd-icon name="close" size="lg" />
        </sd-button>
      </sd-dialog-header>
      <sd-dialog-content>
        <login></login>
      </sd-dialog-content>
      <sd-dialog-footer />
    </sd-dialog>
  </sd-container>
</template>

<script>
import { reactive, toRefs } from "vue";
import Login from "./Login.vue";
export default {
  components: { Login },
  setup() {
    const state = reactive({
      modal: false,
    });

    const handleDialogOpen = () => {
      state.modal = true;
    };

    const handleDialogClose = () => {
      state.modal = false;
    };

    return { ...toRefs(state), handleDialogOpen, handleDialogClose };
  },
};
</script>

<style lang="scss" scoped>
.landing {
  text-align: center;
  position: relative;
  padding-top: 300px;

  a {
    color: var(--primary);
  }

  h2 {
    font-size: 1rem;
  }

  small {
    color: var(--text-light);
  }

  &__signup {
    margin-top: 40px;
    margin-bottom: 8px;
  }
  &__login {
    color: var(--primary);
  }

  img {
    width: 100%;
    height: auto;
    position: absolute;
    top: -326px;
    // z-index: -1;
    opacity: 0.2;
    left: 0;
  }
}
</style>
<template>
  <section class="login-page">
    <sd-container>
      <sd-row class="login">
        <sd-col :lg="3"></sd-col>
        <sd-col :xs="12" :md="12" :lg="6">
          <h1 class="login__header">
            <sd-icon color="var(--success)" name="accessibility_new" size="xl" />
            <br />vestor
          </h1>
          <sd-field
            icon="email"
            required
            placeholder="Username or Email"
            label="Email"
            v-model="user"
          />
          <sd-field
            required
            icon="lock"
            icon-end="remove_red_eye"
            type="password"
            label="Password"
            placeholder="Password"
            v-model="password"
          />
          <sd-button pill block @click="login()">Log in</sd-button>
          <sd-row>
            <sd-col>
              <sd-button class="login__forgot-link" flat>Forgot your username/password?</sd-button>
            </sd-col>
          </sd-row>
          {{user}}
          {{password}}
        </sd-col>
      </sd-row>
    </sd-container>
  </section>
</template>

<script>
import { reactive, toRefs } from "vue";
import router from "../router";
export default {
  components: {},
  setup() {
    const state = reactive({
      user: "",
      password: "",
      remember: false,
    });

    return { ...toRefs(state) };
  },
  methods: {
    login: () => {
      router.push({ name: "Dashboard" });
    },
    test: (e, d) => {},
  },
};
</script>

<style lang="scss" scoped>
.login {
  text-align: center;
  &__header {
    margin-top: 40px;
  }
  &__forgot-link {
    margin-top: 8px;
  }
}
</style>
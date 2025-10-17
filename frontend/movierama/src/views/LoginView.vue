<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "../store/auth";

const router = useRouter();
const authStore = useAuthStore();

const username = ref("");
const password = ref("");
const error = ref("");

async function login() {
  try {
    await authStore.login(username.value, password.value);
    router.push("/");
  } catch (e) {
    error.value = "Invalid username or password";
  }
}
</script>

<template>
  <div class="form-container">
    <h2>Login</h2>
    <form @submit.prevent="login">
      <div class="form-group">
        <label>Username</label>
        <input v-model="username" required />
      </div>
      <div class="form-group">
        <label>Password</label>
        <input type="password" v-model="password" required />
      </div>
      <button type="submit">Sign In</button>
      <p v-if="error" style="color:red">{{ error }}</p>
    </form>
  </div>
</template>


<style scoped>
@import '../styles/form.css';
</style>

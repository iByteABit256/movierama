<template>
  <div class="form-container">
    <h2>Create an Account</h2>
    <form @submit.prevent="register">
      <div class="form-group">
        <label>Username</label>
        <input v-model="username" placeholder="Choose a username" required />
      </div>
      <div class="form-group">
        <label>Email</label>
        <input type="email" v-model="email" placeholder="Your email address" required />
      </div>
      <div class="form-group">
        <label>Password</label>
        <input type="password" v-model="password" placeholder="Create a password" required />
      </div>
      <button type="submit">Register</button>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const username = ref('')
const email = ref('')
const password = ref('')
function register() {
  alert(`Registered as ${username.value}`)
}
</script>

<style scoped>
@import '../styles/form.css';
</style>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../store/auth'

const router = useRouter()
const authStore = useAuthStore()

const username = ref('')
const email = ref('')
const password = ref('')
const message = ref('')

async function register() {
  try {
    await authStore.register(username.value, email.value, password.value)
    router.push('/')
  } catch {
    message.value = 'Failed to register.'
  }
}
</script>

<template>
  <div class="form-container">
    <h2>Register</h2>
    <form @submit.prevent="register">
      <div class="form-group">
        <label>Username</label>
        <input v-model="username" required />
      </div>
      <div class="form-group">
        <label>Email</label>
        <input type="email" v-model="email" required />
      </div>
      <div class="form-group">
        <label>Password</label>
        <input type="password" v-model="password" required />
      </div>
      <button type="submit">Sign Up</button>
      <p>{{ message }}</p>
    </form>
  </div>
</template>

<style scoped>
@import '../styles/form.css';
</style>

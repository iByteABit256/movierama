<template>
  <nav class="navbar">
    <router-link to="/" class="logo">🎬 MovieRama</router-link>

    <div class="links">
      <router-link to="/">Home</router-link>
      <router-link to="/add">Add Movie</router-link>
      <router-link to="/login">Login</router-link>
      <router-link to="/register">Register</router-link>
      <button v-if="authStore.currentUser" @click="logout" class="logout-btn">
        Logout ({{ authStore.currentUser.username }})
      </button>
    </div>
  </nav>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useAuthStore } from '../store/auth'

const router = useRouter()
const authStore = useAuthStore()

const logout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<style scoped>
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background-color: #2c3e50;
}
.logo {
  color: #ecf0f1;
  font-weight: bold;
  font-size: 1.3rem;
  text-decoration: none;
}
.links a {
  color: #ecf0f1;
  margin-left: 1rem;
  text-decoration: none;
}
.links a:hover {
  color: #f1ffe3;
}

.logout-btn {
  margin-left: 1rem;
  background: transparent;
  color: #ff5555;
  border: 1px solid #ff5555;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: 0.2s;
}

.logout-btn:hover {
  background: #ff5555;
  color: white;
}
</style>

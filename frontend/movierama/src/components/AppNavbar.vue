<template>
  <nav class="navbar">
    <router-link to="/" class="logo">🎬 MovieRama</router-link>

    <div class="links">
      <router-link to="/">Home</router-link>
      <router-link to="/add">Add Movie</router-link>
      <router-link to="/login">Login</router-link>
      <router-link to="/register">Register</router-link>
    </div>
  </nav>
</template>

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
</style>

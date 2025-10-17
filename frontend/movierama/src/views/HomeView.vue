<template>
  <div class="home">
    <h2>All Movies</h2>

    <div class="filters">
      <label>Sort by: </label>
      <select v-model="sort">
        <option value="likes">Likes 👍</option>
        <option value="hates">Hates 👎</option>
        <option value="date">Date 📅</option>
      </select>
    </div>

    <MovieCard v-for="m in movies" :key="m.id" :movie="m" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import MovieCard from '../components/MovieCard.vue'

const sort = ref('date')
const movies = ref([
  {
    id: 1,
    title: 'Inception',
    description: 'A mind-bending thriller by Christopher Nolan.',
    username: 'john_doe',
    dateAdded: new Date().toISOString(),
    likes: 120,
    hates: 5,
  },
  {
    id: 2,
    title: 'The Godfather',
    description: 'An all-time classic mafia film.',
    username: 'filmlover',
    dateAdded: new Date().toISOString(),
    likes: 200,
    hates: 3,
  },
])
</script>

<style scoped>
.home {
  max-width: 800px;
  margin: 2rem auto;
}
.filters {
  margin-bottom: 1rem;
}
</style>

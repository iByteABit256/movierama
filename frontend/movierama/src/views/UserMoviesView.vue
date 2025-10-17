<template>
  <div class="user-movies">
    <h2>Movies by {{ username }}</h2>
    <MovieCard v-for="m in movies" :key="m.id" :movie="m" />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import MovieCard from '../components/MovieCard.vue'

const route = useRoute()
const username = route.params.username

const movies = ref([
  {
    id: 1,
    title: 'Interstellar',
    description: 'Exploring space and time.',
    username,
    dateAdded: new Date().toISOString(),
    likes: 89,
    hates: 2,
  },
])
</script>

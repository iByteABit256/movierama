<template>
  <div class="user-movies">
    <h2>Movies by {{ username }}</h2>
    
    <div v-if="moviesStore.loading" class="loading">
      Loading movies...
    </div>
    
    <div v-else-if="movies.length === 0" class="no-movies">
      No movies found for this user.
    </div>
    
    <div v-else class="movies-list">
      <MovieCard 
        v-for="movie in movies" 
        :key="movie.id" 
        :movie="movie" 
      />
    </div>
    
    <div v-if="moviesStore.error" class="error-message">
      {{ moviesStore.error }}
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useMoviesStore } from '../store/movies'
import MovieCard from '../components/MovieCard.vue'

const route = useRoute()
const moviesStore = useMoviesStore()
const username = ref(route.params.username)
const movies = ref([])

const loadUserMovies = async () => {
  try {
    const data = await moviesStore.fetchMoviesByUser(username.value)
    movies.value = data.content || []
  } catch (error) {
    console.error('Failed to load user movies:', error)
    movies.value = []
  }
}

onMounted(() => {
  loadUserMovies()
})

watch(
  () => route.params.username,
  (newUsername) => {
    username.value = newUsername
    moviesStore.clearUserVotes()
    loadUserMovies()
  }
)
</script>

<style scoped>
.user-movies {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.loading {
  text-align: center;
  padding: 40px;
  color: #666;
  font-size: 1.1em;
}

.no-movies {
  text-align: center;
  padding: 40px;
  color: #666;
  font-style: italic;
  font-size: 1.1em;
}
</style>
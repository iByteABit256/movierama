<template>
  <div class="form-container">
    <h2>Add a New Movie</h2>
    <form @submit.prevent="submit">
      <div class="form-group">
        <label>Title</label>
        <input
          v-model="title"
          placeholder="Enter movie title"
          required
          :disabled="moviesStore.loading"
        />
      </div>
      <div class="form-group">
        <label>Description</label>
        <textarea
          v-model="description"
          placeholder="Enter a short description"
          rows="4"
          required
          :disabled="moviesStore.loading"
        ></textarea>
      </div>

      <div v-if="moviesStore.error" class="error-message">
        {{ moviesStore.error }}
      </div>

      <div v-if="successMessage" class="success-message">
        {{ successMessage }}
      </div>

      <button type="submit" :disabled="moviesStore.loading || !title || !description">
        {{ moviesStore.loading ? 'Adding...' : 'Add Movie' }}
      </button>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useMoviesStore } from '../store/movies'

const moviesStore = useMoviesStore()
const title = ref('')
const description = ref('')
const successMessage = ref('')

const submit = async () => {
  successMessage.value = ''

  try {
    await moviesStore.addMovie(title.value, description.value)
    successMessage.value = 'Movie added successfully!'
    title.value = ''
    description.value = ''

    // Clear success message after 3 seconds
    setTimeout(() => {
      successMessage.value = ''
    }, 3000)
  } catch (error) {
    console.error('Failed to add movie:', error)
  }
}
</script>

<style scoped>
@import '../styles/form.css';
</style>

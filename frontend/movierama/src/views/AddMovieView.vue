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

      <transition name="fade">
        <div v-if="showError" class="error-message">
          {{ moviesStore.error }}
        </div>
      </transition>

      <transition name="fade">
        <div v-if="showSuccess" class="success-message">
          {{ successMessage }}
        </div>
      </transition>

      <button type="submit" :disabled="moviesStore.loading || !title || !description">
        {{ moviesStore.loading ? 'Adding...' : 'Add Movie' }}
      </button>
    </form>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useMoviesStore } from '../store/movies'

const moviesStore = useMoviesStore()
const title = ref('')
const description = ref('')
const successMessage = ref('')

const showError = ref(false)
const showSuccess = ref(false)

let errorTimer = null
let successTimer = null

watch(
  () => moviesStore.error,
  (newError) => {
    clearTimeout(errorTimer)
    if (newError) {
      showError.value = true
      errorTimer = setTimeout(() => {
        showError.value = false
        moviesStore.clearError()
      }, 2500)
    } else {
      showError.value = false
    }
  },
)

const submit = async () => {
  successMessage.value = ''
  showSuccess.value = false

  try {
    await moviesStore.addMovie(title.value, description.value)
    successMessage.value = 'Movie added successfully!'
    showSuccess.value = true

    title.value = ''
    description.value = ''

    clearTimeout(successTimer)
    successTimer = setTimeout(() => {
      showSuccess.value = false
      successMessage.value = ''
    }, 2500)
  } catch (error) {
    console.error('Failed to add movie:', error)
  }
}
</script>

<style scoped>
@import '../styles/form.css';
@import '../styles/common.css';
</style>

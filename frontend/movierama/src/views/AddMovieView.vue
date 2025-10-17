<template>
  <div class="form-container">
    <h2>Add a New Movie</h2>
    <form @submit.prevent="submit">
      <div class="form-group">
        <label>Title</label>
        <input v-model="title" placeholder="Enter movie title" required />
      </div>
      <div class="form-group">
        <label>Description</label>
        <textarea
          v-model="description"
          placeholder="Enter a short description"
          rows="4"
          required
        ></textarea>
      </div>
      <button type="submit">Add Movie</button>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const title = ref('')
const description = ref('')
function submit() {
  alert(`Movie added: ${title.value}`)
  title.value = ''
  description.value = ''
}
</script>

<style scoped>
@import '../styles/form.css';
</style>

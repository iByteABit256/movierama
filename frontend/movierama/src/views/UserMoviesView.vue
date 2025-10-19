<template>
  <div class="user-movies">
    <h1>Movies by {{ username }}</h1>

    <div class="sort-controls">
      <label for="sortField">Sort by:</label>
      <select
        id="sortField"
        v-model="sortField"
        @change="onSortChange"
        :disabled="moviesStore.loading"
      >
        <option value="likeCount">Likes</option>
        <option value="hateCount">Hates</option>
        <option value="dateAdded">Date Added</option>
      </select>

      <select
        id="sortDir"
        v-model="sortDirection"
        @change="onSortChange"
        :disabled="moviesStore.loading"
      >
        <option value="desc">Descending</option>
        <option value="asc">Ascending</option>
      </select>
    </div>

    <div class="pagination-controls">
      <label for="userPageSize">Movies per page:</label>
      <select
        id="userPageSize"
        v-model="pageSize"
        @change="onPageSizeChange"
        :disabled="moviesStore.loading"
      >
        <option value="5">5</option>
        <option value="10">10</option>
        <option value="20">20</option>
        <option value="50">50</option>
      </select>
    </div>

    <div v-if="moviesStore.loading" class="loading">Loading movies...</div>

    <div v-else-if="moviesStore.movies.length === 0" class="no-movies">
      No movies found for this user.
    </div>

    <div v-else>
      <div class="movies-list">
        <MovieCard v-for="movie in moviesStore.movies" :key="movie.id" :movie="movie" />
      </div>

      <!-- Pagination -->
      <div class="pagination">
        <div class="pagination-info">
          Showing {{ showingStart }}-{{ showingEnd }} of {{ moviesStore.userTotalElements }} movies
          (Page {{ moviesStore.userCurrentPage + 1 }} of {{ moviesStore.userTotalPages }})
        </div>

        <div class="pagination-buttons">
          <button
            @click="goToFirstPage"
            :disabled="!moviesStore.hasPrevUserPage || moviesStore.loading"
            class="pagination-btn"
          >
            ⏮ First
          </button>
          <button
            @click="prevPage"
            :disabled="!moviesStore.hasPrevUserPage || moviesStore.loading"
            class="pagination-btn"
          >
            ◀ Previous
          </button>

          <div class="page-numbers">
            <button
              v-for="page in visiblePages"
              :key="page"
              @click="goToPage(page - 1)"
              :class="{
                'pagination-btn': true,
                active: page - 1 === moviesStore.userCurrentPage,
              }"
              :disabled="moviesStore.loading"
            >
              {{ page }}
            </button>
            <span v-if="showEllipsis" class="ellipsis">...</span>
          </div>

          <button
            @click="nextPage"
            :disabled="!moviesStore.hasNextUserPage || moviesStore.loading"
            class="pagination-btn"
          >
            Next ▶
          </button>
          <button
            @click="goToLastPage"
            :disabled="!moviesStore.hasNextUserPage || moviesStore.loading"
            class="pagination-btn"
          >
            Last ⏭
          </button>
        </div>
      </div>
    </div>

    <div v-if="moviesStore.error" class="error-message">
      {{ moviesStore.error }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useMoviesStore } from '../store/movies'
import MovieCard from '../components/MovieCard.vue'

const route = useRoute()
const moviesStore = useMoviesStore()
const username = ref(route.params.username)
const pageSize = ref(10)
const sortField = ref(moviesStore.sort.split(',')[0] || 'dateAdded')
const sortDirection = ref(moviesStore.sort.split(',')[1] || 'desc')

const showingStart = computed(() => {
  return moviesStore.userCurrentPage * moviesStore.pageSize + 1
})

const showingEnd = computed(() => {
  const end = (moviesStore.userCurrentPage + 1) * moviesStore.pageSize
  return Math.min(end, moviesStore.userTotalElements)
})

const visiblePages = computed(() => {
  const current = moviesStore.userCurrentPage
  const total = moviesStore.userTotalPages
  const pages = []

  // Show max 5 pages around current page
  let start = Math.max(0, current - 2)
  let end = Math.min(total, start + 5)

  // Adjust start if we're near the end
  if (end - start < 5) {
    start = Math.max(0, end - 5)
  }

  for (let i = start; i < end; i++) {
    pages.push(i + 1)
  }

  return pages
})

const showEllipsis = computed(() => {
  return moviesStore.userTotalPages > visiblePages.value.length
})

const onSortChange = () => {
  moviesStore.sort = `${sortField.value},${sortDirection.value}`
  loadUserMovies()
}

const onPageSizeChange = () => {
  moviesStore.setPageSize(parseInt(pageSize.value))
  loadUserMovies()
}

const nextPage = async () => {
  await moviesStore.nextUserPage(username.value)
}

const prevPage = async () => {
  await moviesStore.prevUserPage(username.value)
}

const goToPage = async (page) => {
  await moviesStore.goToUserPage(username.value, page)
}

const goToFirstPage = async () => {
  await moviesStore.goToUserPage(username.value, 0)
}

const goToLastPage = async () => {
  await moviesStore.goToUserPage(username.value, moviesStore.userTotalPages - 1)
}

const loadUserMovies = async () => {
  try {
    await moviesStore.fetchMoviesByUser(
      username.value,
      moviesStore.userCurrentPage,
      moviesStore.pageSize,
    )
  } catch (error) {
    console.error('Failed to load user movies:', error)
  }
}

onMounted(() => {
  loadUserMovies()
})

// Watch for route changes if the user navigates between different user pages
watch(
  () => route.params.username,
  (newUsername) => {
    username.value = newUsername
    moviesStore.clearUserVotes() // Clear votes when switching users
    loadUserMovies()
  },
)

// Watch for page size changes in store
watch(
  () => moviesStore.pageSize,
  (newSize) => {
    pageSize.value = newSize
  },
)
</script>

<style scoped>
@import '../styles/common.css';
@import '../styles/pagination.css';

.user-movies {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>

import { defineStore } from 'pinia'
import api from '../api/api'
import { useMoviesStore } from './movies'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: localStorage.getItem('token'),
  }),

  getters: {
    currentUser: (state) => {
      if (state.user) return state.user
      if (state.token) {
        try {
          const payload = JSON.parse(atob(state.token.split('.')[1]))
          return { username: payload.sub }
        } catch (error) {
          console.error('Error decoding token:', error)
          return null
        }
      }
      return null
    },
  },

  actions: {
    async login(username, password) {
      const { data } = await api.post('auth/login', { username, password })
      this.token = data.token
      localStorage.setItem('token', data.token)
      this.user = { username }

      const moviesStore = useMoviesStore()
      moviesStore.clearUserVotes()
    },

    async register(username, email, password) {
      const { data } = await api.post('auth/register', { username, email, password })
      this.token = data.token
      localStorage.setItem('token', data.token)
      this.user = { username }

      const moviesStore = useMoviesStore()
      moviesStore.clearUserVotes()
    },

    logout() {
      this.user = null
      this.token = null
      localStorage.removeItem('token')
      delete api.defaults.headers.common['Authorization']
    },
  },
})

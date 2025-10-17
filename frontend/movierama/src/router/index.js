import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import RegisterView from '../views/RegisterView.vue'
import AddMovieView from '../views/AddMovieView.vue'
import UserMoviesView from '../views/UserMoviesView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: HomeView },
    { path: '/login', component: LoginView },
    { path: '/register', component: RegisterView },
    { path: '/add', component: AddMovieView },
    { path: '/user/:username', component: UserMoviesView, props: true },
  ],
})

export default router

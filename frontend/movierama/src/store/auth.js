import { defineStore } from "pinia";
import api from "../api/api";

export const useAuthStore = defineStore("auth", {
  state: () => ({
    user: null,
    token: localStorage.getItem("token"),
  }),

  actions: {
    async login(username, password) {
      const { data } = await api.post("auth/login", { username, password });
      this.token = data.token;
      localStorage.setItem("token", data.token);
      this.user = { username };
    },

    async register(username, email, password) {
      await api.post("auth/register", { username, email, password });
    },

    logout() {
      this.user = null;
      this.token = null;
      localStorage.removeItem("token");
    },
  },
});

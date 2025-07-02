<template>
  <div>
    <h2>System Information</h2>
    <p>OS: {{ systemInfo.os }}</p>
    <p>Architecture: {{ systemInfo.arch }}</p>
    <p>Uptime: {{ systemInfo.uptime }}</p>
    <button @click="fetchSystemInfo">Refresh</button>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      systemInfo: {
        os: '',
        arch: '',
        uptime: ''
      }
    };
  },
  mounted() {
    this.fetchSystemInfo();
  },
  methods: {
    async fetchSystemInfo() {
      try {
        const response = await axios.get('/api/system-info');
        this.systemInfo = response.data;
      } catch (error) {
        console.error('Error fetching system info:', error);
      }
    }
  }
};
</script>

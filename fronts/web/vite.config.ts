import { defineConfig } from 'vite'
import requireTransform from "vite-plugin-require-transform";
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(), 
  ],
  build : {
    commonjsOptions: {
      transformMixedEsModules: true
    }
  }
})

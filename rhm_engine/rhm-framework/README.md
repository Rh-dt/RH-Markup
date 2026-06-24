# RHM Framework

Kerangka kerja markup generasi baru dengan kompilator Rust tingkat rendah. Memungkinkan penulisan komponen kerangka (skeleton) dan optimasi Lighthouse secara inline pada React dan ekosistem Vite.

## Prasyarat Sistem
Paket ini membutuhkan biner rhm_engine yang terpasang pada sistem operasi.

## Instalasi Paket

npm install rhm-framework

## Konfigurasi Vite
Daftarkan plugin RHM ke dalam berkas vite.config.js Anda:

import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { vitePluginRhm } from 'rhm-framework'

export default defineConfig({
  plugins: [react(), vitePluginRhm()],
})

## Cara Penggunaan di React

import { rhm } from 'rhm-framework'

const Dashboard = () => {
  const template = rhm`
    # Area Konten Utama
    ---
    $= rhm.sk(hero)
    $= rhm.lh(img, banner.webp, Banner Utama)
  `

  return <div dangerouslySetInnerHTML={{ __html: template }} />
}

export default Dashboard
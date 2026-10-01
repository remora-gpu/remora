import { defineConfig } from 'vitepress'

const enNav = [
  { text: 'Guide', link: '/architecture' },
  { text: 'Roadmap', link: '/roadmap' },
]

const ruNav = [
  { text: 'Руководство', link: '/ru/architecture' },
  { text: 'Дорожная карта', link: '/ru/roadmap' },
]

export default defineConfig({
  title: 'Remora',
  description: 'Use NVIDIA GPUs over the network, transparently',
  base: '/remora/',
  cleanUrls: true,
  lastUpdated: true,
  srcExclude: ['README.md'],
  themeConfig: {
    socialLinks: [{ icon: 'github', link: 'https://github.com/remora-gpu/remora' }],
    search: { provider: 'local' },
    editLink: {
      pattern: 'https://github.com/remora-gpu/remora/edit/main/docs/:path',
    },
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        nav: enNav,
        sidebar: [
          {
            text: 'Design',
            items: [
              { text: 'Architecture', link: '/architecture' },
              { text: 'Wire protocol', link: '/protocol' },
              { text: 'Threat model', link: '/threat-model' },
            ],
          },
          { text: 'Project', items: [{ text: 'Roadmap', link: '/roadmap' }] },
        ],
      },
    },
    ru: {
      label: 'Русский',
      lang: 'ru',
      themeConfig: {
        nav: ruNav,
        sidebar: [
          {
            text: 'Устройство',
            items: [
              { text: 'Архитектура', link: '/ru/architecture' },
              { text: 'Сетевой протокол', link: '/ru/protocol' },
              { text: 'Модель угроз', link: '/ru/threat-model' },
            ],
          },
          { text: 'Проект', items: [{ text: 'Дорожная карта', link: '/ru/roadmap' }] },
        ],
        outline: { label: 'На этой странице' },
        docFooter: { prev: 'Назад', next: 'Далее' },
      },
    },
  },
})

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useAppStore } from './stores/app';
import Menubar from 'primevue/menubar';
import Dropdown from 'primevue/dropdown';
import Button from 'primevue/button';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import Toast from 'primevue/toast';
import CollectionSidebar from './components/CollectionSidebar.vue';
import RequestTabs from './components/RequestTabs.vue';
import RequestBuilder from './components/RequestBuilder.vue';
import ResponseViewer from './components/ResponseViewer.vue';
import EnvironmentManager from './components/EnvironmentManager.vue';
import EnvironmentDrawer from './components/EnvironmentDrawer.vue';
import HistoryPanel from './components/HistoryPanel.vue';
import SettingsDialog from './components/SettingsDialog.vue';
import type { MenuItem } from 'primevue/menuitem';

const store = useAppStore();

const showEnvManager = ref(false);
const showEnvDrawer = ref(false);
const showHistory = ref(false);
const showSettings = ref(false);

async function handleImportPostman() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    });

    if (selected) {
      const { readTextFile } = await import('@tauri-apps/plugin-fs');
      const content = await readTextFile(selected);
      await store.importPostmanCollection(content);
    }
  } catch (error) {
    console.error('Failed to import collection:', error);
    alert('Failed to import collection: ' + error);
  }
}

const menuItems = ref<MenuItem[]>([
  {
    label: 'File',
    icon: 'pi pi-file',
    items: [
      {
        label: 'New Request',
        icon: 'pi pi-plus',
        command: () => store.resetCurrentRequest(),
      },
      {
        separator: true,
      },
      {
        label: 'Import',
        icon: 'pi pi-download',
        items: [
          {
            label: 'Postman Collection',
            command: () => handleImportPostman(),
          },
        ],
      },
      {
        label: 'Export',
        icon: 'pi pi-upload',
      },
    ],
  },
  {
    label: 'View',
    icon: 'pi pi-eye',
    items: [
      {
        label: 'Environment Variables',
        icon: 'pi pi-box',
        command: () => (showEnvDrawer.value = !showEnvDrawer.value),
      },
      {
        label: 'History',
        icon: 'pi pi-history',
        command: () => (showHistory.value = true),
      },
    ],
  },
  {
    label: 'Settings',
    icon: 'pi pi-cog',
    command: () => (showSettings.value = true),
  },
]);

const selectedEnv = computed({
  get: () => store.activeEnvironment?.id || null,
  set: async (value) => {
    if (value) {
      await store.setActiveEnvironment(value);
    }
  },
});

onMounted(() => {
  // Add dark mode class to body
  document.documentElement.classList.add('dark-mode');
});
</script>

<template>
  <div class="app dark-mode">
    <Toast />

    <div class="app-header">
      <div class="header-left">
        <h1 class="app-title">
          <img src="/fetchr.svg" alt="Fetchr" class="app-logo" />
          Fetchr
        </h1>
        <Menubar :model="menuItems" class="app-menu" />
      </div>
      <div class="header-right">
        <Button
          :icon="showEnvDrawer ? 'pi pi-angle-right' : 'pi pi-angle-left'"
          text
          rounded
          @click="showEnvDrawer = !showEnvDrawer"
          v-tooltip.left="showEnvDrawer ? 'Hide Environment Variables' : 'Show Environment Variables'"
          :severity="showEnvDrawer ? 'success' : 'secondary'"
        />
        <Dropdown
          v-model="selectedEnv"
          :options="store.environments"
          optionLabel="name"
          optionValue="id"
          placeholder="No Environment"
          class="env-dropdown"
        >
          <template #value="{ value }">
            <span v-if="value">
              <i class="pi pi-box"></i>
              {{ store.environments.find((e) => e.id === value)?.name }}
            </span>
            <span v-else class="placeholder">
              <i class="pi pi-box"></i>
              No Environment
            </span>
          </template>
          <template #footer>
            <div class="env-dropdown-footer">
              <Button
                label="Manage Environments"
                icon="pi pi-cog"
                text
                size="small"
                class="w-full"
                @click="showEnvManager = true"
              />
            </div>
          </template>
        </Dropdown>
      </div>
    </div>

    <div class="app-content">
      <Splitter style="height: 100%">
        <SplitterPanel :size="20" :minSize="15">
          <CollectionSidebar />
        </SplitterPanel>
        <SplitterPanel :size="showEnvDrawer ? 60 : 80">
          <div class="main-panel">
            <RequestTabs />
            <Splitter layout="vertical" style="height: 100%">
              <SplitterPanel :size="50" :minSize="30">
                <RequestBuilder />
              </SplitterPanel>
              <SplitterPanel :size="50" :minSize="30">
                <ResponseViewer />
              </SplitterPanel>
            </Splitter>
          </div>
        </SplitterPanel>
        <SplitterPanel v-if="showEnvDrawer" :size="20" :minSize="15">
          <EnvironmentDrawer :visible="showEnvDrawer" @toggle="showEnvDrawer = !showEnvDrawer" />
        </SplitterPanel>
      </Splitter>
    </div>

    <EnvironmentManager v-model:visible="showEnvManager" />
    <HistoryPanel v-model:visible="showHistory" />
    <SettingsDialog v-model:visible="showSettings" />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
    Cantarell, 'Helvetica Neue', sans-serif;
}

.dark-mode {
  color-scheme: dark;
}

.w-full {
  width: 100%;
}
</style>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--surface-ground);
  color: var(--text-color);
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  background: var(--surface-section);
  border-bottom: 1px solid var(--surface-border);
  gap: 1rem;
  position: relative;
  z-index: 1000;
  flex-wrap: nowrap;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex: 1;
  min-width: 0;
  overflow: visible;
}

.app-title {
  font-size: 1.25rem;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--primary-color);
  margin: 0;
  white-space: nowrap;
  flex-shrink: 0;
}

.app-logo {
  width: 24px;
  height: 24px;
}

.app-menu {
  border: none;
  background: transparent;
  padding: 0;
  flex-shrink: 0;
}

/* Hide mobile menu button completely */
.app-menu :deep(.p-menubar-button) {
  display: none !important;
}

/* Force desktop menu to always show */
.app-menu :deep(.p-menubar-root-list) {
  display: flex !important;
  position: static !important;
  background: transparent !important;
  border: none !important;
  padding: 0 !important;
  flex-wrap: nowrap !important;
  white-space: nowrap !important;
}

.app-menu :deep(.p-menuitem) {
  flex-shrink: 0 !important;
}

.app-menu :deep(.p-submenu-list) {
  background: var(--surface-overlay) !important;
  border: 1px solid var(--surface-border) !important;
}

.app-menu :deep(.p-menuitem-link) {
  color: var(--text-color) !important;
}

.app-menu :deep(.p-menuitem-link:hover),
.app-menu :deep(.p-menuitem-link:focus) {
  background: var(--surface-100) !important;
}

.app-menu :deep(.p-menuitem-icon) {
  color: var(--text-color) !important;
}

.app-menu :deep(.p-menuitem-text) {
  color: var(--text-color) !important;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 1;
  min-width: 200px;
}

.env-dropdown {
  min-width: 200px;
}

.env-dropdown-footer {
  padding: 0.5rem;
  border-top: 1px solid var(--surface-border);
  background: var(--surface-section);
}

.env-dropdown-footer :deep(.p-button) {
  justify-content: flex-start;
  color: var(--text-color);
}

.env-dropdown-footer :deep(.p-button:hover) {
  background: var(--surface-hover);
}

.env-dropdown .placeholder {
  color: var(--text-color-secondary);
}

.app-content {
  flex: 1;
  overflow: hidden;
}

.main-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}
</style>
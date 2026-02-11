<script setup lang="ts">
import { useAppStore } from '../stores/app';
import Button from 'primevue/button';

const store = useAppStore();

const getMethodColor = (method: string) => {
  const colors: Record<string, string> = {
    GET: '#10b981',
    POST: '#3b82f6',
    PUT: '#f59e0b',
    DELETE: '#ef4444',
    PATCH: '#8b5cf6',
    HEAD: '#6b7280',
    OPTIONS: '#6b7280',
  };
  return colors[method] || '#6b7280';
};

const handleTabClick = (index: number) => {
  store.switchTab(index);
};

const handleCloseTab = (event: Event, index: number) => {
  event.stopPropagation();
  store.closeTab(index);
};

const handleNewTab = () => {
  store.createNewTab();
};
</script>

<template>
  <div class="request-tabs-container">
    <div class="tabs-list">
      <div
        v-for="(tab, index) in store.openTabs"
        :key="tab.id"
        :class="['tab', { active: index === store.activeTabIndex }]"
        @click="handleTabClick(index)"
      >
        <span class="tab-method" :style="{ color: getMethodColor(tab.request.method) }">
          {{ tab.request.method }}
        </span>
        <span class="tab-name">{{ tab.name }}</span>
        <span v-if="tab.isDirty" class="tab-dirty">●</span>
        <Button
          icon="pi pi-times"
          text
          rounded
          size="small"
          class="tab-close"
          @click="(e) => handleCloseTab(e, index)"
          v-tooltip.bottom="'Close tab'"
        />
      </div>
      <Button
        icon="pi pi-plus"
        text
        rounded
        size="small"
        class="new-tab-btn"
        @click="handleNewTab"
        v-tooltip.bottom="'New tab'"
      />
    </div>
  </div>
</template>

<style scoped>
.request-tabs-container {
  display: flex;
  background: var(--surface-section);
  border-bottom: 1px solid var(--surface-border);
  flex-shrink: 0;
}

.tabs-list {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem;
  overflow-x: auto;
  flex: 1;
}

.tab {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--surface-ground);
  border: 1px solid var(--surface-border);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 150px;
  max-width: 250px;
  position: relative;
}

.tab:hover {
  background: var(--surface-hover);
}

.tab.active {
  background: var(--surface-card);
  border-color: var(--primary-400);
}

.tab-method {
  font-weight: 600;
  font-size: 0.75rem;
  flex-shrink: 0;
}

.tab-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.875rem;
  color: var(--text-color);
}

.tab-dirty {
  color: var(--primary-400);
  font-size: 1.2rem;
  line-height: 0;
  flex-shrink: 0;
}

.tab-close {
  flex-shrink: 0;
  opacity: 0.6;
  transition: opacity 0.2s;
}

.tab-close:hover {
  opacity: 1;
}

.new-tab-btn {
  flex-shrink: 0;
  color: var(--text-color-secondary);
}

.new-tab-btn:hover {
  color: var(--primary-400);
}

/* Scrollbar styling */
.tabs-list::-webkit-scrollbar {
  height: 4px;
}

.tabs-list::-webkit-scrollbar-track {
  background: var(--surface-ground);
}

.tabs-list::-webkit-scrollbar-thumb {
  background: var(--surface-border);
  border-radius: 4px;
}

.tabs-list::-webkit-scrollbar-thumb:hover {
  background: var(--text-color-secondary);
}
</style>

<script setup lang="ts">
import { computed } from 'vue';
import { useAppStore } from '../stores/app';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';

const store = useAppStore();

defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  toggle: [];
}>();

const environmentVariables = computed(() => {
  if (!store.activeEnvironment) return [];
  try {
    return JSON.parse(store.activeEnvironment.variables || '[]');
  } catch {
    return [];
  }
});

const environmentName = computed(() => store.activeEnvironment?.name || 'No Environment');
</script>

<template>
  <div class="env-sidebar">
    <div class="sidebar-header">
      <h3>Environment Variables</h3>
      <Button icon="pi pi-times" text rounded @click="emit('toggle')" v-tooltip.left="'Close sidebar'" />
    </div>

    <div class="env-drawer-content">
      <div v-if="store.activeEnvironment" class="env-info">
        <div class="env-name">
          <i class="pi pi-box"></i>
          {{ environmentName }}
        </div>
        <div class="var-count">{{ environmentVariables.length }} variable{{ environmentVariables.length !== 1 ? 's' : '' }}</div>
      </div>

      <div v-if="!store.activeEnvironment" class="no-env">
        <i class="pi pi-info-circle"></i>
        <p>No environment selected</p>
        <p class="hint">Select an environment from the dropdown to view its variables</p>
      </div>

      <DataTable v-else :value="environmentVariables" class="variables-table" :paginator="false" scrollable scroll-height="flex">
        <Column field="key" header="Key">
          <template #body="{ data }">
            <span class="var-key">{{ data.key }}</span>
          </template>
        </Column>
        <Column field="value" header="Value">
          <template #body="{ data }">
            <span class="var-value">{{ data.value }}</span>
          </template>
        </Column>
      </DataTable>
    </div>
  </div>
</template>

<style scoped>
.env-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--surface-ground);
  border-left: 1px solid var(--surface-border);
  overflow: hidden;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--surface-border);
  background: var(--surface-section);
  flex-shrink: 0;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-color);
}

.env-drawer-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  flex: 1;
  overflow: hidden;
}

.env-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem;
  background: var(--surface-section);
  border-radius: 6px;
  border: 1px solid var(--surface-border);
}

.env-name {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
  font-size: 1.1rem;
  color: var(--primary-400);
}

.var-count {
  font-size: 0.875rem;
  color: var(--text-color-secondary);
}

.no-env {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 3rem 1rem;
  text-align: center;
  color: var(--text-color-secondary);
}

.no-env .pi {
  font-size: 3rem;
  opacity: 0.5;
}

.no-env .hint {
  font-size: 0.875rem;
  opacity: 0.7;
}

.variables-table {
  flex: 1;
  overflow: auto;
}

.var-key {
  font-family: 'Courier New', monospace;
  color: var(--primary-400);
  font-weight: 500;
}

.var-value {
  font-family: 'Courier New', monospace;
  color: var(--text-color);
}
</style>

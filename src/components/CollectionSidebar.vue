<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useAppStore } from '../stores/app';
import Tree from 'primevue/tree';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Dialog from 'primevue/dialog';
import ContextMenu from 'primevue/contextmenu';
import type { TreeNode } from '../types';
import type { MenuItem } from 'primevue/menuitem';

const store = useAppStore();

const searchQuery = ref('');
const showNewDialog = ref(false);
const showSaveDialog = ref(false);
const newItemName = ref('');
const newItemType = ref<'folder' | 'request'>('folder');
const selectedNode = ref<TreeNode | null>(null);
const contextMenuRef = ref();

const saveRequestName = ref('');
const saveCollectionId = ref('');

const contextMenuItems = computed<MenuItem[]>(() => {
  const items: MenuItem[] = [
    {
      label: 'New Request',
      icon: 'pi pi-file',
      command: () => createNewRequest(),
    },
    {
      label: 'New Folder',
      icon: 'pi pi-folder',
      command: () => createNewFolder(),
    },
  ];

  // Only show export for folders
  if (selectedNode.value?.type === 'folder') {
    items.push(
      {
        separator: true,
      },
      {
        label: 'Export',
        icon: 'pi pi-upload',
        command: () => exportNode(),
      }
    );
  }

  items.push(
    {
      separator: true,
    },
    {
      label: 'Delete',
      icon: 'pi pi-trash',
      command: () => deleteNode(),
    }
  );

  return items;
});

function onNodeSelect(node: TreeNode) {
  if (node.type === 'request') {
    store.loadRequest(node.key);
  }
}


function onNodeRightClickDirect(event: MouseEvent, node: TreeNode) {
  event.preventDefault();
  event.stopPropagation();
  console.log('Direct right click on node:', node);
  selectedNode.value = node;
  if (contextMenuRef.value) {
    contextMenuRef.value.show(event);
  }
}

function createNewFolder() {
  newItemType.value = 'folder';
  newItemName.value = '';
  showNewDialog.value = true;
}

function createNewRequest() {
  newItemType.value = 'request';
  newItemName.value = '';
  showNewDialog.value = true;
}

async function handleCreate() {
  if (!newItemName.value) return;

  const parentId = selectedNode.value?.type === 'folder' ? selectedNode.value.key : undefined;

  if (newItemType.value === 'folder') {
    await store.createCollection(newItemName.value, parentId, true);
  } else {
    await store.createCollection(newItemName.value, parentId, false);
  }

  showNewDialog.value = false;
}

async function deleteNode() {
  if (!selectedNode.value) return;

  if (selectedNode.value.type === 'folder') {
    await store.deleteCollection(selectedNode.value.key);
  } else {
    const request = selectedNode.value.data as any;
    await store.deleteRequest(selectedNode.value.key, request.collection_id);
  }
}

async function exportNode() {
  if (!selectedNode.value || selectedNode.value.type !== 'folder') return;

  try {
    const json = await store.exportCollection(selectedNode.value.key);

    // Use Tauri file dialog to save
    const { save } = await import('@tauri-apps/plugin-dialog');
    const filePath = await save({
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }],
      defaultPath: `${selectedNode.value.label}.json`
    });

    if (filePath) {
      const { writeTextFile } = await import('@tauri-apps/plugin-fs');
      await writeTextFile(filePath, json);

      // Show success message
      const { message } = await import('@tauri-apps/plugin-dialog');
      await message('Collection exported successfully!', {
        title: 'Export Success',
        kind: 'info'
      });
    }
  } catch (error) {
    console.error('Failed to export collection:', error);
    const { message } = await import('@tauri-apps/plugin-dialog');
    await message(`Failed to export collection: ${error}`, {
      title: 'Export Error',
      kind: 'error'
    });
  }
}

function showSaveRequestDialog() {
  saveRequestName.value = '';
  saveCollectionId.value = store.collections[0]?.id || '';
  showSaveDialog.value = true;
}

async function handleSaveRequest() {
  if (!saveRequestName.value || !saveCollectionId.value) return;
  await store.saveCurrentRequest(saveRequestName.value, saveCollectionId.value);
  showSaveDialog.value = false;
}

onMounted(() => {
  store.initialize();
});
</script>

<template>
  <div class="collection-sidebar">
    <div class="sidebar-header">
      <h3>Collections</h3>
      <div class="header-actions">
        <Button
          icon="pi pi-plus"
          text
          rounded
          @click="createNewFolder"
          v-tooltip.right="'New Folder'"
        />
        <Button
          icon="pi pi-save"
          text
          rounded
          @click="showSaveRequestDialog"
          v-tooltip.right="'Save Request'"
        />
      </div>
    </div>

    <div class="search-box">
      <span class="p-input-icon-left w-full">
        <i class="pi pi-search"></i>
        <InputText
          v-model="searchQuery"
          placeholder="Search collections..."
          class="w-full"
        />
      </span>
    </div>

    <div class="tree-container">
      <Tree
        :value="store.collectionTree"
        selectionMode="single"
        @nodeSelect="onNodeSelect"
        class="collection-tree"
      >
        <template #default="{ node }">
          <span
            :class="{ 'font-bold': node.type === 'folder' }"
            @contextmenu="(e: MouseEvent) => onNodeRightClickDirect(e, node as TreeNode)"
          >
            {{ node.label }}
          </span>
        </template>
      </Tree>
    </div>

    <ContextMenu ref="contextMenuRef" :model="contextMenuItems" />

    <!-- New Item Dialog -->
    <Dialog
      v-model:visible="showNewDialog"
      :header="`New ${newItemType === 'folder' ? 'Folder' : 'Request'}`"
      :modal="true"
      :style="{ width: '400px' }"
    >
      <div class="dialog-content">
        <label>Name</label>
        <InputText
          v-model="newItemName"
          placeholder="Enter name"
          class="w-full"
          @keyup.enter="handleCreate"
        />
      </div>
      <template #footer>
        <Button label="Cancel" text @click="showNewDialog = false" />
        <Button label="Create" @click="handleCreate" />
      </template>
    </Dialog>

    <!-- Save Request Dialog -->
    <Dialog
      v-model:visible="showSaveDialog"
      header="Save Request"
      :modal="true"
      :style="{ width: '400px' }"
    >
      <div class="dialog-content">
        <div class="field">
          <label>Request Name</label>
          <InputText
            v-model="saveRequestName"
            placeholder="Enter request name"
            class="w-full"
          />
        </div>
        <div class="field">
          <label>Collection</label>
          <select v-model="saveCollectionId" class="w-full">
            <option
              v-for="collection in store.collections"
              :key="collection.id"
              :value="collection.id"
            >
              {{ collection.name }}
            </option>
          </select>
        </div>
      </div>
      <template #footer>
        <Button label="Cancel" text @click="showSaveDialog = false" />
        <Button label="Save" @click="handleSaveRequest" />
      </template>
    </Dialog>
  </div>
</template>

<style scoped>
.collection-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--surface-section);
  border-right: 1px solid var(--surface-border);
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--surface-border);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1.125rem;
}

.header-actions {
  display: flex;
  gap: 0.25rem;
}

.search-box {
  padding: 0.75rem;
  border-bottom: 1px solid var(--surface-border);
}

.tree-container {
  flex: 1;
  overflow: auto;
  padding: 0.5rem;
}

.collection-tree {
  border: none;
  background: transparent;
}

.dialog-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem 0;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.field label {
  font-weight: 500;
  font-size: 0.875rem;
}

.field select {
  padding: 0.5rem;
  border: 1px solid var(--surface-border);
  border-radius: 6px;
  background: var(--surface-overlay);
  color: var(--text-color);
}
</style>

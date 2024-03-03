<script setup lang="ts">
import { Theme } from '@/pages/Settings/ViewModel';
import { PropType, ref } from 'vue';

const emit = defineEmits(["onClick"]);

const props = defineProps({
    text: {
        required: true,
        type: String
    },
    checked: {
        required: true,
        type: Boolean
    },
    theme: {
        required: true,
        type: Object as PropType<Theme>
    }
});

const accentPrimary = ref(props.theme.accent_primary);

</script>

<template>
    <div class="flex items-center cursor-pointer" @click="emit('onClick')" >
        <div>{{ text }}</div>
        <div class=" ml-2 mt-[1px] h-[24px] w-[24px] background-tertiary rounded-full radio-border p-1">
            <div v-if="checked" class=" w-full h-full background-primary rounded-full"></div>
        </div>
    </div>
</template>

<style scoped>
.radio-border {
    border: 2px solid v-bind(accentPrimary);
}
</style>
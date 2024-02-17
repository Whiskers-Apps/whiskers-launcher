import { StoreThemeSelectValue, selectValues } from "./ThemesStore.vue";

export function updateStoreThemeSelectValue(index: number, value: string) {
let newValues: StoreThemeSelectValue[] = [];
let found = false;

selectValues.value.forEach((selectValue) => {
if (selectValue.themeIndex === index) {
found = true;

newValues.push({
themeIndex: index,
value: value,
});
} else {
newValues.push(selectValue);
}
});

if (!found) {
selectValues.value.push({});
}

selectValues.value = newValues;
console.log(selectValues.value);
}

<script setup lang="ts">
import { h } from 'vue'
import { useRoute, useRouter } from 'vue-router'
const route = useRoute()
const router = useRouter()
const { params, query } = route
const { path } = params
router.replace({ path: '/' + path, query })
</script>
    
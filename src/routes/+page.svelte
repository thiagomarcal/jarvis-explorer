<script lang="ts">

    import {invoke} from "@tauri-apps/api/tauri";


    async function listFiles(path: string) {
        return await invoke("list_files", {path})
    }


    type Entry = {
        name: string;
        is_dir: boolean;
        size: number;
        created: string;
        modified: string;
    };

    const parseDate = (date_in_sec_epoch) => {
        let d = new Date(0);
        d.setUTCSeconds(date_in_sec_epoch)
        return d.toDateString()
    }


    async function doSearch() {
        let result = await listFiles(inputValue)
        console.log(result);
        entries = result
            .filter(item => Boolean(item.Ok))
            .map(item => item.Ok)
            .map(item => ({
                name: item.filename,
                is_dir: item.is_dir,
                size: item.size,
                created: parseDate(item.created.secs_since_epoch),
                modified: parseDate(item.modified.secs_since_epoch)
            }))
    }

    let entries: Entry[] = [];

    let inputValue = ""


</script>

<div class="flex flex-col gap-4 p-4">
    <div class="card p-4 basis-1/4">
        <div class="flex flex-row flex-grow gap-3 p-3 h-full justify-center items-center">
            <div class="flex flex-row gap-3 basis-1/12 justify-center">
                <div>
                    <button type="button" class="btn-icon variant-filled">
                        <i class="fa-solid fa-arrow-left"></i>
                    </button>
                </div>
                <div>
                    <button type="button" class="btn-icon variant-filled">
                        <i class="fa-solid fa-arrow-right"></i>
                    </button>
                </div>
            </div>

            <div class="basis-3/4">
                <input class="input p-3" type="search" name="demo" bind:value={inputValue}
                       placeholder="Search...">
            </div>
            <div class="flex flex-row gap-3 basis-1/12 justify-center">
                <button type="button" class="btn-icon variant-filled" on:click={doSearch}>
                    <i class="fa-solid fa-skull"></i>
                </button>
            </div>


        </div>
    </div>
    <div class="card basis-3/4 p-4 flex flex-row" >
        <div class="table-container max-h-screen overflow-y-auto">
            <!-- Native Table Element -->
            <table class="table table-hover table-fixed">
                <thead>
                <tr>
                    <th>Name</th>
                    <th>Is Directory</th>
                    <th>Size</th>
                    <th>Created</th>
                    <th>Modified</th>
                </tr>
                </thead>
                <tbody>
                {#each entries as entry}
                    <tr>
                        <td class="truncate ...">{entry.name}</td>
                        <td>{entry.is_dir}</td>
                        <td>{entry.size}</td>
                        <td>{entry.created}</td>
                        <td>{entry.modified}</td>
                    </tr>
                {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>

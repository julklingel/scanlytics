<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";

    export let title: string
    export let description: string
    export let link: string


   

  </script>

  <a href="{link}"> 
  <Card.Root class="w-[350px]">
    <Card.Header>
      <Card.Title>{title}</Card.Title>
      <Card.Description>{description}</Card.Description>
    </Card.Header>
    <Card.Content>

    </Card.Content>
    <Card.Footer class="flex justify-between">
    </Card.Footer>
  </Card.Root>
</a>
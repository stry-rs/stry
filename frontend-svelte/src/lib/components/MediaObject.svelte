<script lang="ts"></script>

<div class="flex">
	<div>
		<slot name="tile"></slot>
	</div>
	<div class="flex-1 flex flex-col">
		<div class="text-base">
			<slot name="title"></slot>
		</div>
		<div class="text-sm">
			<slot name="sub"></slot>
		</div>
	</div>
	<div>
		<slot name="meta"></slot>
	</div>
</div>

<script lang="ts">
    import Card from "../../components/ui/card.svelte";
    import { Label } from "$lib/components/ui/label/index.js";
    import Button from "$lib/components/ui/button/button.svelte";


</script>

<form id="patientNoteForm" class="max-w-lg mx-auto mt-10 p-6 bg-white rounded-lg shadow-md">
    <div class="grid grid-cols-1 gap-6">
 
      <div>
        <label for="patientName" class="block text-sm font-medium text-gray-700">Patient Name</label>
        <input type="text" id="patientName" name="patientName" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="patientId" class="block text-sm font-medium text-gray-700">Patient ID</label>
        <input type="text" id="patientId" name="patientId" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="symptoms" class="block text-sm font-medium text-gray-700">Symptoms</label>
        <textarea id="symptoms" name="symptoms" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"></textarea>
      </div>
  
      <div>
        <label for="diagnosis" class="block text-sm font-medium text-gray-700">Diagnosis</label>
        <textarea id="diagnosis" name="diagnosis" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"></textarea>
      </div>
  
      <div>
        <label for="treatment" class="block text-sm font-medium text-gray-700">Treatment</label>
        <textarea id="treatment" name="treatment" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"></textarea>
      </div>
  
      <div>
        <label for="createdAt" class="block text-sm font-medium text-gray-700">Created At</label>
        <input type="datetime-local" id="createdAt" name="createdAt" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="updatedAt" class="block text-sm font-medium text-gray-700">Updated At</label>
        <input type="datetime-local" id="updatedAt" name="updatedAt" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="followUpDate" class="block text-sm font-medium text-gray-700">Follow-up Date</label>
        <input type="date" id="followUpDate" name="followUpDate" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="severity" class="block text-sm font-medium text-gray-700">Severity</label>
        <select id="severity" name="severity" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
          <option value="Low">Low</option>
          <option value="Medium">Medium</option>
          <option value="High">High</option>
        </select>
      </div>
  
      <div class="flex items-center">
        <input type="checkbox" id="isUrgent" name="isUrgent" class="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
        <label for="isUrgent" class="ml-2 block text-sm text-gray-900">Is Urgent</label>
      </div>
  
      <div>
        <label for="department" class="block text-sm font-medium text-gray-700">Department</label>
        <input type="text" id="department" name="department" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
  
      <div>
        <label for="attendingDoctor" class="block text-sm font-medium text-gray-700">Attending Doctor</label>
        <input type="text" id="attendingDoctor" name="attendingDoctor" required class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
      </div>
    </div>
  
    <div class="mt-6">
      <Button type="submit" class="w-full px-4 py-2">
        Submit
      </Button>
    </div>
  </form>
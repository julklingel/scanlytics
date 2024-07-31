import { test, expect } from '@playwright/test';


test('submit patient note form', async ({ page }) => {
  await page.goto('/notes/new');

  // Fill out the form
  await page.fill('input[name="patientName"]', 'John Doe');
  await page.fill('input[name="patientId"]', 'JD001');
  await page.fill('textarea[name="symptoms"]', 'Fever, cough');
  await page.fill('textarea[name="diagnosis"]', 'Common cold');
  await page.fill('textarea[name="treatment"]', 'Rest and fluids');


  await page.fill('input[name="department"]', 'General Medicine');
  await page.fill('input[name="attendingDoctor"]', 'Dr. Smith');


  await page.click('button[type="submit"]');


});



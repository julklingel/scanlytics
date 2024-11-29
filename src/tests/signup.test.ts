import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import Signup from "../routes/auth/signup/+page.svelte";
import { render, fireEvent } from "@testing-library/svelte";
import { toast } from "svelte-sonner";
import * as navigation from "$app/navigation";

vi.mock('@melt-ui/svelte', () => ({
  createDialog: () => ({
    open: { subscribe: vi.fn() },
    trigger: { 'data-state': '' },
    overlay: { 'data-state': '' },
    content: { 'data-state': '' },
    title: { id: '' },
    description: { id: '' },
    close: vi.fn(),
  }),
  getFocusTrap: () => ({
    activate: vi.fn(),
    deactivate: vi.fn(),
  }),
  melt: vi.fn(),
}));

// Mock tabbable
vi.mock('tabbable', () => ({
  tabbable: () => [],
  focusable: () => [],
  isFocusable: () => true,
  isTabbable: () => true,
}));

// Mock focus-trap
vi.mock('focus-trap', () => ({
  default: () => ({
    activate: vi.fn(),
    deactivate: vi.fn(),
  }),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock('svelte-sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

vi.mock('mode-watcher', () => ({
  mode: {
    subscribe: (fn: (value: string) => void) => {
      fn('light');
      return { unsubscribe: vi.fn() };
    },
  },
}));

vi.mock('$app/navigation', () => ({
  goto: vi.fn(() => Promise.resolve()),
}));


describe("Signup Component", () => {
    // To prevent Type Error 
    beforeAll(() => {
      // @ts-ignore
      Element.prototype.querySelectorAll = function() { return []; };
    });
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllMocks();
    mockIPC(<T>(cmd: string, payload?: any): Promise<T> => {
      if (cmd === "signup") {
        return Promise.resolve({ success: true } as T);
      }
      return Promise.reject("Command not found");
    });

    const portalRoot = document.createElement('div');
    portalRoot.setAttribute('id', 'portal');
    document.body.appendChild(portalRoot);
  });

  afterEach(() => {
    vi.useRealTimers();
    document.getElementById('portal')?.remove();
  });


  it("renders signup form correctly", async () => {
    const { container } = render(Signup);

    const nameInput = container.querySelector("#name");
    const emailInput = container.querySelector("#email");
    const passwordInput = container.querySelector("#password");
    const confirmPasswordInput = container.querySelector("#confirm-password");
    const submitButton = container.querySelector('button[type="submit"]');
    const logo = container.querySelector('img[alt="Logo"]');

    expect(nameInput).toBeTruthy();
    expect(nameInput?.getAttribute("placeholder")).toBe("Enter your full name");
    expect(emailInput).toBeTruthy();
    expect(emailInput?.getAttribute("placeholder")).toBe("Enter your email");
    expect(passwordInput).toBeTruthy();
    expect(passwordInput?.getAttribute("placeholder")).toBe("Create a password");
    expect(confirmPasswordInput).toBeTruthy();
    expect(confirmPasswordInput?.getAttribute("placeholder")).toBe("Confirm your password");
    expect(submitButton).toBeTruthy();
    expect(logo).toBeTruthy();
    expect(logo?.getAttribute("class")).toContain("h-12 w-12");
  });

  it("shows error when submitting empty form", async () => {
    const { container } = render(Signup);
    const form = container.querySelector("form");
    await fireEvent.submit(form!);
    expect(toast.error).toHaveBeenCalledWith("Please fill in all fields.");
  });

  it("shows progress bar during signup attempt", async () => {
    const { container } = render(Signup);
    const form = container.querySelector("form");
    
    const nameInput = container.querySelector("#name") as HTMLInputElement;
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector("#password") as HTMLInputElement;
    const confirmPasswordInput = container.querySelector("#confirm-password") as HTMLInputElement;

    await fireEvent.input(nameInput, { target: { value: "Test User" } });
    await fireEvent.input(emailInput, { target: { value: "test@example.com" } });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });
    await fireEvent.input(confirmPasswordInput, { target: { value: "password123" } });
    
    await fireEvent.submit(form!);

    const progressBar = container.querySelector('[id="progressbar"]');
    expect(progressBar).toBeTruthy();
  });

  it("updates button text during loading state", async () => {
    const { container } = render(Signup);
    const form = container.querySelector("form");
    
    const nameInput = container.querySelector("#name") as HTMLInputElement;
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector("#password") as HTMLInputElement;
    const confirmPasswordInput = container.querySelector("#confirm-password") as HTMLInputElement;

    await fireEvent.input(nameInput, { target: { value: "Test User" } });
    await fireEvent.input(emailInput, { target: { value: "test@example.com" } });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });
    await fireEvent.input(confirmPasswordInput, { target: { value: "password123" } });

    const submitButton = container.querySelector('button[type="submit"]');
    expect(submitButton?.textContent).toBe("Sign Up");

    await fireEvent.submit(form!);
    expect(submitButton?.textContent).toBe("Signing Up...");
  });

  it("navigates to landing page when logo is clicked", async () => {
    const { container } = render(Signup);
    const logoButton = container.querySelector('button:has(img[alt="Logo"])');

    const gotoSpy = vi.spyOn(navigation, "goto").mockResolvedValue();

    await fireEvent.click(logoButton!);

    expect(gotoSpy).toHaveBeenCalledWith("/");
  });



  it("handles failed signup", async () => {
    vi.clearAllMocks();
    
    const mockError = { message: "Email already exists" };
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockRejectedValue(mockError);

    const { container } = render(Signup);
    

    const form = container.querySelector("form");
    const nameInput = container.querySelector("#name") as HTMLInputElement;
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector("#password") as HTMLInputElement;
    const confirmPasswordInput = container.querySelector("#confirm-password") as HTMLInputElement;
    const submitButton = container.querySelector('button[type="submit"]');

    await fireEvent.input(nameInput, { target: { value: "Test User" } });
    await fireEvent.input(emailInput, { target: { value: "test@example.com" } });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });
    await fireEvent.input(confirmPasswordInput, { target: { value: "password123" } });

    await fireEvent.submit(form!);
    await vi.runAllTimers();
    await vi.advanceTimersByTimeAsync(1000);


    expect(toast.error).toHaveBeenCalledWith(JSON.stringify(mockError));
    expect(submitButton?.textContent).toBe("Sign Up");
  }, { timeout: 10000 });

  it("renders login link for existing users", () => {
    const { container } = render(Signup);
    const loginLink = container.querySelector('a[href="/auth/login"]');
    const loginText = container.querySelector('p');
    
    expect(loginLink).toBeTruthy();
    expect(loginText?.textContent).toBe("Already have an account?");
  });
  it("successfully signs up a user and shows verification dialog", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockResolvedValue({ success: true });

    const { container } = render(Signup);
    
  
    const form = container.querySelector("form");
    const nameInput = container.querySelector("#name") as HTMLInputElement;
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector("#password") as HTMLInputElement;
    const confirmPasswordInput = container.querySelector("#confirm-password") as HTMLInputElement;
    
    
    await fireEvent.input(nameInput, { target: { value: "Test User" } });
    await fireEvent.input(emailInput, { target: { value: "test@example.com" } });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });
    await fireEvent.input(confirmPasswordInput, { target: { value: "password123" } });
    

    await fireEvent.submit(form!);
    
 
    await vi.runAllTimers();
    await vi.advanceTimersByTimeAsync(500);
    
  
    expect(invoke).toHaveBeenCalledWith("signup", {
      signupData: JSON.stringify({
        full_name: "Test User",
        user_email: "test@example.com",
        password: "password123",
        confirm_password: "password123"
      })
    });
    

    await vi.advanceTimersByTimeAsync(0);
    

    const dialogContent = document.querySelector('[id="dialog"]');
    expect(dialogContent).toBeInTheDocument();
    
    const dialogTitle = dialogContent?.querySelector('h2');
    expect(dialogTitle).toHaveTextContent("Thank you for signing up!");
    

    
    const loginButton = dialogContent?.querySelector('a[href="/auth/login"]');
    expect(loginButton).toBeInTheDocument();
  });


  
});

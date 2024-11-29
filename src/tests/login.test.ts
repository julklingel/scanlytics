import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import Login from "../routes/auth/login/+page.svelte";
import { render, fireEvent } from "@testing-library/svelte";
import { goto } from "$app/navigation";
import { toast } from "svelte-sonner";
import * as navigation from "$app/navigation";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("Login Component", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.clearAllMocks();
    mockIPC(<T>(cmd: string, payload?: any): Promise<T> => {
      if (cmd === "login") {
        return Promise.resolve({ success: true } as T);
      }
      return Promise.reject("Command not found");
    });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders login form correctly", async () => {
    const { container } = render(Login);

    const emailInput = container.querySelector("#email");
    const passwordInput = container.querySelector("#password");
    const submitButton = container.querySelector('button[type="submit"]');
    const logo = container.querySelector('img[alt="Logo"]');
    const signupText = container.querySelector("p");

    expect(emailInput).toBeTruthy();
    expect(emailInput?.getAttribute("placeholder")).toBe("Enter your email");
    expect(passwordInput).toBeTruthy();
    expect(passwordInput?.getAttribute("placeholder")).toBe(
      "Enter your password"
    );
    expect(submitButton).toBeTruthy();
    expect(logo).toBeTruthy();
    expect(logo?.getAttribute("class")).toContain("h-12 w-12");
  });

  it("shows error when submitting empty form", async () => {
    const { container } = render(Login);
    const form = container.querySelector("form");
    await fireEvent.submit(form!);
    expect(toast.error).toHaveBeenCalledWith("Please fill in all fields.");
  });

  it("shows progress bar during login attempt", async () => {
    const { container } = render(Login);
    const form = container.querySelector("form");
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector(
      "#password"
    ) as HTMLInputElement;

    await fireEvent.input(emailInput, {
      target: { value: "test@example.com" },
    });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });
    await fireEvent.submit(form!);

    const progressBar = container.querySelector('[id="progressbar"]');
    expect(progressBar).toBeTruthy();
  });

  it("updates button text during loading state", async () => {
    const { container } = render(Login);
    const form = container.querySelector("form");
    const emailInput = container.querySelector("#email") as HTMLInputElement;
    const passwordInput = container.querySelector(
      "#password"
    ) as HTMLInputElement;

    await fireEvent.input(emailInput, {
      target: { value: "test@example.com" },
    });
    await fireEvent.input(passwordInput, { target: { value: "password123" } });

    const submitButton = container.querySelector('button[type="submit"]');
    expect(submitButton?.textContent).toBe("Login");

    await fireEvent.submit(form!);
    expect(submitButton?.textContent).toBe("Logging in...");
  });

  it("renders forgot password component", () => {
    const { container } = render(Login);
    const forgotPasswordElement = container.querySelector(".flex.justify-end");
    expect(forgotPasswordElement).toBeTruthy();
  });

  it("renders sign up button with correct link", () => {
    const { container } = render(Login);
    const signUpButton = container.querySelector('a[href="/auth/signup"]');
    expect(signUpButton).toBeTruthy();
  });

  it("navigates to landing page when logo is clicked", async () => {
    const { container } = render(Login);
    const logoButton = container.querySelector('button:has(img[alt="Logo"])');

    const gotoSpy = vi.spyOn(navigation, "goto").mockResolvedValue();

    await fireEvent.click(logoButton!);

    expect(gotoSpy).toHaveBeenCalledWith("/");
  });

  it(
    "handles failed login with wrong password",
    async () => {
      vi.clearAllMocks();

      const mockLoginError = "Invalid credentials";
      const { invoke } = await import("@tauri-apps/api/core");
      vi.mocked(invoke).mockRejectedValue(mockLoginError);

      const { container } = render(Login);

      const form = container.querySelector("form");
      const emailInput = container.querySelector("#email") as HTMLInputElement;
      const passwordInput = container.querySelector(
        "#password"
      ) as HTMLInputElement;
      const submitButton = container.querySelector('button[type="submit"]');

      await fireEvent.input(emailInput, {
        target: { value: "test@example.com" },
      });
      await fireEvent.input(passwordInput, {
        target: { value: "wrongpassword" },
      });

      await fireEvent.submit(form!);

      await vi.runAllTimers();

      await vi.advanceTimersByTimeAsync(1000);

      expect(toast.error).toHaveBeenCalledWith(mockLoginError);
      expect(submitButton?.textContent).toBe("Login");
    },
    { timeout: 10000 }
  );
  it(
    "handles successful login",
    async () => {
      vi.clearAllMocks();
      const { invoke } = await import("@tauri-apps/api/core");
      vi.mocked(invoke).mockResolvedValue({ success: true });
      const gotoSpy = vi.spyOn(navigation, "goto").mockResolvedValue();
      const { container } = render(Login);

      const form = container.querySelector("form");
      const emailInput = container.querySelector("#email") as HTMLInputElement;
      const passwordInput = container.querySelector(
        "#password"
      ) as HTMLInputElement;
      const submitButton = container.querySelector('button[type="submit"]');

      await fireEvent.input(emailInput, {
        target: { value: "test@example.com" },
      });
      await fireEvent.input(passwordInput, {
        target: { value: "correctpassword" },
      });

      await fireEvent.submit(form!);
      expect(submitButton?.textContent).toBe("Logging in...");
      const progressBar = container.querySelector('[id="progressbar"]');
      expect(progressBar).toBeTruthy();

      await vi.runAllTimers();
      await vi.advanceTimersByTimeAsync(1000);
      expect(toast.success).toHaveBeenCalledWith("Login successful!");
      expect(gotoSpy).toHaveBeenCalledWith("/menu");
    },
    { timeout: 10000 }
  );
});

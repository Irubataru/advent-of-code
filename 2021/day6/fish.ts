export class LanternFish {
  private timer: number;

  constructor(timer: number) {
    this.timer = timer;
  }

  public nextDay(): LanternFish | null {
    if (this.timer > 0) {
      this.timer = this.timer - 1;
      return null;
    }

    this.timer = 6;
    return new LanternFish(8);
  }
}

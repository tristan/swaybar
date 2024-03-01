# swaybar

My own personal swaybar implementation.

Implements the [swaybar protocol](https://www.mankier.com/7/swaybar-protocol) for use in [sway](https://swaywm.org/).

## Modules

 - work time / break reminder : gently reminds me to stand up and get away from the screen after a set period of time (uses the wayland protocol `org_kde_kwin_idle` to get the idle timeouts).
 - battery usage
 - current date/time
 - connected wifi

## will this work for anyone other than me?

maybe? If you don't have the same battery setup as me (i.e. the same layout in `/sys/class/power_supply/`) then probably not.

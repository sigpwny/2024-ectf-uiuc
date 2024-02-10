# *******************************************************************************
# Set the target microcontroller and board to compile for.  

# Every TARGET microcontroller has some Board Support Packages (BSPs) that are 
# available for it under the MaximSDK/Libraries/Boards/TARGET folder.  The BSP 
# that gets selected is MaximSDK/Libraries/Boards/TARGET/BOARD.

# Configuration Variables:
# - TARGET : Override the default target microcontroller.  Ex: TARGET=MAX78000
# - BOARD : Override the default BSP (case sensitive).  Ex: BOARD=EvKit_V1, BOARD=FTHR_RevA


ifeq "$(TARGET)" ""
# Default target microcontroller
TARGET := MAX78000
TARGET_UC := MAX78000
TARGET_LC := max78000
else
# "TARGET" has been overridden in the environment or on the command-line.
# We need to calculate an upper and lowercase version of the part number,
# because paths on Linux and MacOS are case-sensitive.
TARGET_UC := $(subst m,M,$(subst a,A,$(subst x,X,$(TARGET))))
TARGET_LC := $(subst M,m,$(subst A,a,$(subst X,x,$(TARGET))))
endif

# Default board.
BOARD = FTHR_RevA
MFLOAT_ABI = soft

# *******************************************************************************
# Locate the MaximSDK

# This Makefile needs to know where to find the MaximSDK, and the MAXIM_PATH variable 
# should point to the root directory of the MaximSDK installation.  Setting this manually
# is usually only required if you're working on the command-line.

# If MAXIM_PATH is not specified, we assume the project still lives inside of the MaximSDK 
# and move up from this project's original location.

# Configuration Variables:
# - MAXIM_PATH : Tell this Makefile where to find the MaximSDK.  Ex:  MAXIM_PATH=C:/MaximSDK


ifneq "$(MAXIM_PATH)" ""
# Sanitize MAXIM_PATH for backslashes
MAXIM_PATH := $(subst \,/,$(MAXIM_PATH))
# Locate some other useful paths...
LIBS_DIR := $(abspath $(MAXIM_PATH)/Libraries)
CMSIS_ROOT := $(LIBS_DIR)/CMSIS
endif

# *******************************************************************************
# Include project Makefile.  We do this after formulating TARGET, BOARD, and MAXIM_PATH 
# in case project.mk needs to reference those values.  However, we also include
# this as early as possible in the Makefile so that it can append to or override
# the variables below.


# PROJECTMK ?= $(abspath ./project.mk)
# include $(PROJECTMK)
# $(info Loaded project.mk)
# PROJECTMK is also used by implicit rules and other libraries to add project.mk as a watch file

# *******************************************************************************
# Final path sanitization and re-calculation.  No options here.

ifeq "$(MAXIM_PATH)" ""
# MAXIM_PATH is still not defined...
DEPTH := ../../../
MAXIM_PATH := $(abspath $(DEPTH))
$(warning Warning:  MAXIM_PATH is not set!  Set MAXIM_PATH in your environment or in project.mk to clear this warning.)
$(warning Warning:  Attempting to use $(MAXIM_PATH) calculated from relative path)
else
# Sanitize MAXIM_PATH for backslashes
MAXIM_PATH := $(subst \,/,$(MAXIM_PATH))
endif

# Final recalculation of LIBS_DIR/CMSIS_ROOT
LIBS_DIR := $(abspath $(MAXIM_PATH)/Libraries)
CMSIS_ROOT := $(LIBS_DIR)/CMSIS

# One final UC/LC check in case user set TARGET in project.mk
TARGET_UC := $(subst m,M,$(subst a,A,$(subst x,X,$(TARGET))))
TARGET_LC := $(subst M,m,$(subst A,a,$(subst X,x,$(TARGET))))

export TARGET
export TARGET_UC
export TARGET_LC
export CMSIS_ROOT




# *******************************************************************************



# Where to find source files for this project.
VPATH += .
VPATH := $(VPATH)
VPATH += $(CMSIS_ROOT)/Device/Maxim/$(TARGET_UC)/Source

# Where to find header files for this project
IPATH += .
IPATH += $(LIBS_DIR)/MiscDrivers/LED
IPATH := $(IPATH)

AUTOSEARCH ?= 1
ifeq ($(AUTOSEARCH), 1)
# Auto-detect all C source files on VPATH
SRCS += $(wildcard $(addsuffix /*.c, $(VPATH)))
endif

# Collapse SRCS before passing them on to the next stage
SRCS := $(SRCS)

# *******************************************************************************
# Compiler options

# Configuration Variables:
# - DEBUG : Set DEBUG=1 to build explicitly for debugging.  This adds some additional
#			symbols and sets -Og as the default optimization level.
# - MXC_OPTIMIZE_CFLAGS : Override the default compiler optimization level.  
#			Ex: MXC_OPTIMIZE_CFLAGS = -O2
# - PROJ_CFLAGS : Add additional compiler flags to the build.
#			You should use the "+=" operator with this option. 
#			Ex:  PROJ_CFLAGS += -Wextra
# - MFLOAT_ABI : Set the floating point acceleration level.
#			The only options are "hard", "soft", or "softfp".
#			Ex: MFLOAT_ABI = hard
# - LINKERFILE : Override the default linkerfile.
#			Ex: LINKERFILE = customlinkerfile.ld
# - LINKERPATH : Override the default search location for $(LINKERFILE)
#			The default search location is $(CMSIS_ROOT)/Device/Maxim/$(TARGET_UC)/Source/GCC
#			If $(LINKERFILE) cannot be found at this path, then the root project
#			directory will be used as a fallback.

# Select 'GCC' or 'IAR' compiler
ifeq "$(COMPILER)" ""
COMPILER := GCC
endif

# Set default compiler optimization levels
ifeq "$(MAKECMDGOALS)" "release"
# Default optimization level for "release" builds (make release)
MXC_OPTIMIZE_CFLAGS ?= -O2
DEBUG = 0
endif

ifeq ($(DEBUG),1)
# Optimizes for debugging as recommended
# by GNU for code-edit-debug cycles
# https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html#Optimize-Options
MXC_OPTIMIZE_CFLAGS := -Og
endif

# Default level if not building for release or explicitly for debug
MXC_OPTIMIZE_CFLAGS ?= -O2

# Set compiler flags
PROJ_CFLAGS += -Wall # Enable warnings

# *******************************************************************************
# Secure Boot Tools (SBT)
SBT ?= 0
ifeq ($(SBT), 1)
MAXIM_SBT_DIR ?= $(MAXIM_PATH)/Tools/SBT
MAXIM_SBT_DIR := $(subst \,/,$(MAXIM_SBT_DIR))
# ^ Must sanitize path for \ on Windows, since this may come from an environment
# variable.

export MAXIM_SBT_DIR # SBTs must have this environment variable defined to work
endif # SBT

MAKECMDGOALS := all

# Enable colors when --sync-output is used.
# See https://www.gnu.org/software/make/manual/make.html#Terminal-Output (section 13.2)
ifneq ($(MAKE_TERMOUT),)
PROJ_CFLAGS += -fdiagnostics-color=always
endif

ifneq ($(FORCE_COLOR),)
PROJ_CFLAGS += -fdiagnostics-color=always
endif


include $(LIBS_DIR)/libs.mk
# Include the peripheral driver
PERIPH_DRIVER_DIR=$(LIBS_DIR)/PeriphDrivers
include $(PERIPH_DRIVER_DIR)/periphdriver.mk
export PERIPH_DRIVER_DIR
################################################################################
# Include the rules for building for this target. All other makefiles should be
# included before this one.
include $(CMSIS_ROOT)/Device/Maxim/$(TARGET_UC)/Source/$(COMPILER)/$(TARGET_LC).mk

all:
# 	Extend the functionality of the "all" recipe here
	arm-none-eabi-size --format=berkeley $(BUILD_DIR)/$(PROJECT).elf

# Make wrapper but with included libraries
wrapper: wrapper.c
	arm-none-eabi-gcc -c -o wrapper.o wrapper.c -I $(LIBS) -I $(PERIPH_DRIVER_LIBS) $(CFLAGS) $(PROJ_CFLAGS) -I $(IPATH) $(VPATH)
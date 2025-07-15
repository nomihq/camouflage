

package main

import (
	"fmt"

	"github.com/progrium/darwinkit/macos/coreaudio"
)

func main() {
	fmt.Println("Attempting to interact with Core Audio...")

	var size uint32
	err := coreaudio.AudioHardwareGetPropertyInfo(coreaudio.KAudioHardwarePropertyDevices, &size, nil)
	if err != nil {
		panic(err)
	}

	numDevices := int(size) / 4
	deviceIDs := make([]coreaudio.AudioDeviceID, numDevices)

	err = coreaudio.AudioHardwareGetProperty(coreaudio.KAudioHardwarePropertyDevices, &size, &deviceIDs[0])
	if err != nil {
		panic(err)
	}

	fmt.Printf("Found %d audio devices:\n", numDevices)
	for _, deviceID := range deviceIDs {
		var size uint32 = 256
		var name [256]byte
		address := coreaudio.AudioObjectPropertyAddress{
			MSelector: coreaudio.KAudioObjectPropertyElementName,
			MScope:    coreaudio.KAudioObjectPropertyScopeGlobal,
			MElement:  coreaudio.KAudioObjectPropertyElementMain,
		}
		err = coreaudio.AudioObjectGetPropertyData(uint32(deviceID), &address, 0, nil, &size, &name[0])
		if err == nil {
			fmt.Printf("  - %s\n", string(name[:size-1]))
		}
	}
}


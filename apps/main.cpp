#include <AudioToolbox/AudioToolbox.h>
#include <iostream>
#include "properties.hpp"
#include "PedalBoard.hpp"

int main() {
    AudioComponentDescription desc = {};
    desc.componentType = kAudioUnitType_Output;
    desc.componentSubType = kAudioUnitSubType_DefaultOutput;
    desc.componentManufacturer = kAudioUnitManufacturer_Apple;

    AudioComponent comp = AudioComponentFindNext(nullptr, &desc);
    if(!comp) {
        std::cerr << "Could not find default output unit.\n";
        return -1;
    }

    AudioUnit audioUnit;
    if(AudioComponentInstanceNew(comp, &audioUnit) != noErr) {
        std::cerr << "Could not create Audio Unit Instance!\n";
        return -1;
    }

    AudioUnitInitialize(audioUnit);

    PedalBoard tempPedalBoard;
    tempPedalBoard.setupAudioUnit(audioUnit);

    AudioOutputUnitStart(audioUnit);

    std::cout << "Press [ENTER] to stop playback and exit...\n";
    std::cin.get();

    AudioOutputUnitStop(audioUnit);
    AudioUnitUninitialize(audioUnit);
    AudioComponentInstanceDispose(audioUnit);

    return 0;
}
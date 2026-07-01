#pragma once
#include "AudioToolbox/AudioToolbox.h"

class PedalBoard {
private:
    /*
    void* inRefCon - the same pointer as inputProcRefCon
    AudioUnitRenderActionFlags* ioActionFlags - needed flags
    const AudioTimeStamp* inTimeStamp - timing structure, exact host time for given audio
    [[maybe_unused]] UInt32 inBusNumber - which audio bus is currently asking for data. (Default = 0)
    UInt32 inNumberFrames - amount of audio samples you must generate during single callback cycle
    AudioBufferList* ioData - pointer to the memory buffers allocated in operating system
    for stereo - ioData->mBuffers[0] is left channel, ioData->mBuffers[1] is right channel
    */
    static OSStatus RenderCallback(
        void* inRefCon,
        AudioUnitRenderActionFlags* ioActionFlags,
        const AudioTimeStamp* inTimeStamp,
        [[maybe_unused]] UInt32 inBusNumber,
        UInt32 inNumberFrames,
        AudioBufferList* ioData);

    /*
    [[maybe_unused]] AudioUnitRenderActionFlags* ioActionFlags - flags (could be helpful in the future)
    [[maybe_unused]] const AudioTimeStamp* inTimeStamp - CPU host time and exact sample frame number for when specific audio will hit the speakers
    [[maybe_unused]] UInt32 inNumberFrames - exact number of audio samples the operating system is demanding right now
    AudioBufferList* ioData - input/output data, pointer to raw memory provided by the physical audio hardware
    */
    OSStatus renderAudio(
        [[maybe_unused]] AudioUnitRenderActionFlags* ioActionFlags,
        [[maybe_unused]] const AudioTimeStamp* inTimeStamp,
        [[maybe_unused]] UInt32 inNumberFrames,
        AudioBufferList* ioData);

public:
    //AudioUnit audioUnit - apple's standard format for a software audio plugin or node (piece of a music gear)
    void setupAudioUnit(AudioUnit audioUnit);
};
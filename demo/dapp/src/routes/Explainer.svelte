<script>
    import { BasePage, RebaseLogo, Button } from "components";
    import { useNavigate } from "svelte-navigator";

    const navigate = useNavigate();
</script>

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
        <RebaseLogo class="w-full flex items-center justify-center my-[16px]" logo/>
        <div>
            <p>
                Rebase is a decentralized service that allows people and organizations
                to map their identities to cryptographic keys using accounts in a 
                user-centric and publicly auditable way. This instance allows users 
                to incrementally build their identity through various workflows for 
                social media accounts, self-attested information, and more.  
            </p>
            <br />
            <p>
                Credentials are issued as W3C Verifiable Credentials to a user's 
                decentralized identifier, and are saved locally unless a storage system 
                is provided. This instance begins with a user's Ethereum account as the 
                core identifier, with various credentials being issued to that account
                as the subject. In the future, this could be augmented with different
                identifiers, workflows, and more. 
            </p>
        </div>
        <div class="w-full flex justify-center items-center">
            <Button
                class="w-full max-w-42 my-[16px] hover:scale-105"
                onClick={() => navigate("/account")}
                text="Enter"
                action
            />
        </div>
    </div>
</BasePage>
